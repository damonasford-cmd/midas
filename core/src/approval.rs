use crate::action::{
    ActionRiskProfile,
    ApprovalRequest,
    ApprovalStatus,
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ApprovalManager {
    pending: Vec<ApprovalRequest>,
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    pub fn create(
        &mut self,
        action: impl Into<String>,
        profile: ActionRiskProfile,
        reason: impl Into<String>,
    ) -> ApprovalRequest {
        let request =
            ApprovalRequest::new(
                action,
                profile,
                reason,
            );

        self.pending.push(
            request.clone(),
        );

        request
    }

    pub fn insert(
        &mut self,
        request: ApprovalRequest,
    ) -> Uuid {
        let id = request.id;

        self.pending.push(request);

        id
    }

    pub fn get(
        &self,
        id: Uuid,
    ) -> Option<&ApprovalRequest> {
        self.pending
            .iter()
            .find(|request| request.id == id)
    }

    pub fn get_mut(
        &mut self,
        id: Uuid,
    ) -> Option<&mut ApprovalRequest> {
        self.pending
            .iter_mut()
            .find(|request| request.id == id)
    }

    pub fn approve(
        &mut self,
        id: Uuid,
    ) -> bool {
        if let Some(request) =
            self.get_mut(id)
        {
            if request.status
                == ApprovalStatus::Pending
            {
                request.approve();
                return true;
            }
        }

        false
    }

    pub fn refuse(
        &mut self,
        id: Uuid,
    ) -> bool {
        if let Some(request) =
            self.get_mut(id)
        {
            if request.status
                == ApprovalStatus::Pending
            {
                request.refuse();
                return true;
            }
        }

        false
    }

    pub fn cancel(
        &mut self,
        id: Uuid,
    ) -> bool {
        if let Some(request) =
            self.get_mut(id)
        {
            if request.status
                == ApprovalStatus::Pending
            {
                request.cancel();
                return true;
            }
        }

        false
    }

    pub fn expire(
        &mut self,
        id: Uuid,
    ) -> bool {
        if let Some(request) =
            self.get_mut(id)
        {
            if request.status
                == ApprovalStatus::Pending
            {
                request.expire();
                return true;
            }
        }

        false
    }

    pub fn pending(
        &self,
    ) -> Vec<&ApprovalRequest> {
        self.pending
            .iter()
            .filter(|request| {
                request.status
                    == ApprovalStatus::Pending
            })
            .collect()
    }

    pub fn all(
        &self,
    ) -> &[ApprovalRequest] {
        &self.pending
    }

    pub fn remove_finished(
        &mut self,
    ) {
        self.pending
            .retain(|request| {
                request.status
                    == ApprovalStatus::Pending
            });
    }

    pub fn count_pending(
        &self,
    ) -> usize {
        self.pending()
            .len()
    }

    pub fn is_pending(
        &self,
        id: Uuid,
    ) -> bool {
        self.get(id)
            .map(|request| {
                request.status
                    == ApprovalStatus::Pending
            })
            .unwrap_or(false)
    }
}
