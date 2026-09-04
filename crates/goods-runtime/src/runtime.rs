//! Runtime entry point for Observe → Assess, Observe → Identify Need, and
//! Identify Need → Request/Receive Care.

use std::error::Error as StdError;

use goods_application::ports::human_feedback_source::HumanFeedbackSource;
use goods_application::ports::observation_source::ObservationSource;
use goods_application::use_cases::assess_state::AssessState;
use goods_application::use_cases::identify_need::IdentifyNeed;
use goods_application::use_cases::observe_goods::ObserveGoods;
use goods_application::use_cases::receive_care::ReceiveCare;
use goods_application::use_cases::request_care::RequestCare;
use goods_domain::care::{CareAction, CareRequest};
use goods_domain::goods::Goods;
use goods_domain::need::NeedAssessment;
use goods_domain::state::GoodsState;

/// The result of observing, identifying Need and, when needed, requesting
/// and receiving Care.
pub type CareOutcome = (GoodsState, NeedAssessment, Option<CareRequest>, Option<CareAction>);

/// Product-independent composition of the Phase 1, Phase 2 and Phase 3 use
/// cases.
pub struct GoodsRuntime<S> {
    source: S,
}

impl<S> GoodsRuntime<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

impl<S: ObservationSource> GoodsRuntime<S> {
    pub fn observe_and_assess(&self, goods: &Goods) -> Result<GoodsState, S::Error> {
        let observation = ObserveGoods::execute(&self.source, goods)?;
        Ok(AssessState::execute(goods, observation))
    }

    /// Observe once and derive both the Health Assessment and the Need
    /// Assessment from the same observation.
    pub fn observe_and_identify_needs(
        &self,
        goods: &Goods,
    ) -> Result<(GoodsState, NeedAssessment), S::Error> {
        let observation = ObserveGoods::execute(&self.source, goods)?;
        let needs = IdentifyNeed::execute(goods, &observation);
        let state = AssessState::execute(goods, observation);
        Ok((state, needs))
    }

    /// Observe, identify needs, and — only when a Need exists — request Care
    /// and record the resulting Care Action from Human Feedback. The Human
    /// Feedback content always comes from `feedback_source`; the runtime
    /// never decides or invents it.
    pub fn request_care<F>(
        &self,
        goods: &Goods,
        feedback_source: &F,
    ) -> Result<CareOutcome, Box<dyn StdError>>
    where
        F: HumanFeedbackSource,
        F::Error: StdError + 'static,
        S::Error: StdError + 'static,
    {
        let (state, needs) = self.observe_and_identify_needs(goods)?;
        let request = RequestCare::execute(&needs);
        let action = match &request {
            Some(request) => {
                let feedback = feedback_source.provide_feedback(request)?;
                Some(ReceiveCare::execute(request.clone(), feedback))
            }
            None => None,
        };

        Ok((state, needs, request, action))
    }
}
