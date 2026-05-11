use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModelId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorkerId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimeNamespace(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueueClass {
    Realtime,
    Interactive,
    Batch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmissionDecision {
    pub admitted: bool,
    pub queue_class: QueueClass,
    pub reason: Option<String>,
}

impl AdmissionDecision {
    pub fn admitted(queue_class: QueueClass) -> Self {
        Self {
            admitted: true,
            queue_class,
            reason: None,
        }
    }

    pub fn rejected(reason: impl Into<String>) -> Self {
        Self {
            admitted: false,
            queue_class: QueueClass::Batch,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkerState {
    Starting,
    Available,
    Draining,
    Unhealthy,
    Offline,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkerHealth {
    pub state: WorkerState,
    pub last_heartbeat_ms: u64,
    pub message: Option<String>,
}

impl WorkerHealth {
    pub fn available(last_heartbeat_ms: u64) -> Self {
        Self {
            state: WorkerState::Available,
            last_heartbeat_ms,
            message: None,
        }
    }

    pub fn can_accept_new_work(&self) -> bool {
        self.state == WorkerState::Available
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkerLoad {
    pub queued_requests: u32,
    pub running_requests: u32,
    pub waiting_tokens: u64,
    pub kv_bytes_used: u64,
    pub kv_bytes_capacity: u64,
    pub estimated_decode_tokens_per_second: f64,
}

impl WorkerLoad {
    pub fn kv_utilization(&self) -> f64 {
        if self.kv_bytes_capacity == 0 {
            return 1.0;
        }
        self.kv_bytes_used as f64 / self.kv_bytes_capacity as f64
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkerDescriptor {
    pub worker_id: WorkerId,
    pub runtime_namespace: RuntimeNamespace,
    pub model_ids: Vec<ModelId>,
    pub supported_capabilities: Vec<String>,
    pub health: WorkerHealth,
    pub load: WorkerLoad,
}

impl WorkerDescriptor {
    pub fn serves_model(&self, model_id: &ModelId) -> bool {
        self.model_ids.iter().any(|candidate| candidate == model_id)
    }

    pub fn supports_capabilities(&self, capabilities: &[String]) -> bool {
        capabilities
            .iter()
            .all(|capability| self.supported_capabilities.contains(capability))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutingRequest {
    pub request_id: RequestId,
    pub model_id: ModelId,
    pub prompt_tokens: u32,
    pub max_new_tokens: u32,
    pub priority: u8,
    pub required_capabilities: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteTarget {
    pub worker_id: WorkerId,
    pub runtime_namespace: RuntimeNamespace,
    pub estimated_wait_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutingDecision {
    pub request_id: RequestId,
    pub target: RouteTarget,
    pub alternatives: Vec<RouteTarget>,
    pub decision_reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouterErrorKind {
    AdmissionRejected,
    ModelUnavailable,
    CapabilityUnavailable,
    NoHealthyWorker,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouterError {
    pub kind: RouterErrorKind,
    pub message: String,
}

impl RouterError {
    pub fn new(kind: RouterErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

pub trait RoutingPolicy {
    fn select(
        &self,
        request: &RoutingRequest,
        workers: &[WorkerDescriptor],
    ) -> Result<RoutingDecision, RouterError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeastLoadedPolicy {
    pub max_kv_utilization: f64,
}

impl Default for LeastLoadedPolicy {
    fn default() -> Self {
        Self {
            max_kv_utilization: 0.95,
        }
    }
}

impl LeastLoadedPolicy {
    fn score(&self, worker: &WorkerDescriptor) -> f64 {
        let load = &worker.load;
        (load.running_requests as f64 * 1_000.0)
            + (load.queued_requests as f64 * 100.0)
            + (load.waiting_tokens as f64)
            + (load.kv_utilization() * 10_000.0)
            - (load.estimated_decode_tokens_per_second * 10.0)
    }

    fn route_target(&self, worker: &WorkerDescriptor) -> RouteTarget {
        let estimated_wait_ms =
            (worker.load.queued_requests as u64 * 50) + (worker.load.waiting_tokens / 10);
        RouteTarget {
            worker_id: worker.worker_id.clone(),
            runtime_namespace: worker.runtime_namespace.clone(),
            estimated_wait_ms,
        }
    }
}

impl RoutingPolicy for LeastLoadedPolicy {
    fn select(
        &self,
        request: &RoutingRequest,
        workers: &[WorkerDescriptor],
    ) -> Result<RoutingDecision, RouterError> {
        let model_candidates: Vec<&WorkerDescriptor> = workers
            .iter()
            .filter(|worker| worker.serves_model(&request.model_id))
            .collect();

        if model_candidates.is_empty() {
            return Err(RouterError::new(
                RouterErrorKind::ModelUnavailable,
                format!("no worker serves model {}", request.model_id.0),
            ));
        }

        let capability_candidates: Vec<&WorkerDescriptor> = model_candidates
            .into_iter()
            .filter(|worker| worker.supports_capabilities(&request.required_capabilities))
            .collect();

        if capability_candidates.is_empty() {
            return Err(RouterError::new(
                RouterErrorKind::CapabilityUnavailable,
                "no worker satisfies required capabilities",
            ));
        }

        let mut candidates: Vec<&WorkerDescriptor> = capability_candidates
            .into_iter()
            .filter(|worker| worker.health.can_accept_new_work())
            .filter(|worker| worker.load.kv_utilization() <= self.max_kv_utilization)
            .collect();

        if candidates.is_empty() {
            return Err(RouterError::new(
                RouterErrorKind::NoHealthyWorker,
                "no healthy worker has available KV capacity",
            ));
        }

        candidates.sort_by(|left, right| {
            self.score(left)
                .partial_cmp(&self.score(right))
                .unwrap_or(Ordering::Equal)
        });

        let target = self.route_target(candidates[0]);
        let alternatives = candidates
            .iter()
            .skip(1)
            .map(|worker| self.route_target(worker))
            .collect();

        Ok(RoutingDecision {
            request_id: request.request_id.clone(),
            target,
            alternatives,
            decision_reason: "least-loaded healthy worker".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worker(name: &str, running_requests: u32, queued_requests: u32) -> WorkerDescriptor {
        WorkerDescriptor {
            worker_id: WorkerId(name.to_string()),
            runtime_namespace: RuntimeNamespace("cuda-prod".to_string()),
            model_ids: vec![ModelId("deepseek-v4-flash".to_string())],
            supported_capabilities: vec!["tp8".to_string(), "cuda".to_string()],
            health: WorkerHealth::available(1_000),
            load: WorkerLoad {
                queued_requests,
                running_requests,
                waiting_tokens: 0,
                kv_bytes_used: 10,
                kv_bytes_capacity: 100,
                estimated_decode_tokens_per_second: 40.0,
            },
        }
    }

    fn request() -> RoutingRequest {
        RoutingRequest {
            request_id: RequestId("req-1".to_string()),
            model_id: ModelId("deepseek-v4-flash".to_string()),
            prompt_tokens: 16,
            max_new_tokens: 32,
            priority: 100,
            required_capabilities: vec!["tp8".to_string(), "cuda".to_string()],
        }
    }

    #[test]
    fn selects_least_loaded_worker() {
        let policy = LeastLoadedPolicy::default();
        let decision = policy
            .select(&request(), &[worker("busy", 4, 2), worker("idle", 0, 0)])
            .unwrap();

        assert_eq!(decision.target.worker_id, WorkerId("idle".to_string()));
        assert_eq!(decision.alternatives.len(), 1);
    }

    #[test]
    fn rejects_when_model_is_unavailable() {
        let policy = LeastLoadedPolicy::default();
        let mut request = request();
        request.model_id = ModelId("unknown".to_string());

        let err = policy.select(&request, &[worker("w1", 0, 0)]).unwrap_err();
        assert_eq!(err.kind, RouterErrorKind::ModelUnavailable);
    }

    #[test]
    fn rejects_when_kv_is_full() {
        let policy = LeastLoadedPolicy::default();
        let mut full = worker("full", 0, 0);
        full.load.kv_bytes_used = 100;

        let err = policy.select(&request(), &[full]).unwrap_err();
        assert_eq!(err.kind, RouterErrorKind::NoHealthyWorker);
    }
}

