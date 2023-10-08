use freenet_stdlib::contract_composition::{
    ContractComponent, ParametersComponent, SummaryComponent,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatRoom {
    pub members: Vec<dependency_2::ChatRoomMember>,
    pub messages: Vec<dependency_1::SignedComposable<dependency_2::ChatRoomMessage>>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRoomParameters {
    pub owner_public_key: dependency_2::PublicKey,
}

impl ParametersComponent for ChatRoomParameters {
    fn contract_id(&self) -> Option<freenet_stdlib::prelude::ContractInstanceId> {
        unimplemented!()
    }
}

impl<'x> From<&'x ChatRoomParameters> for dependency_2::ChatRoomMsgParameters {
    fn from(_: &'x ChatRoomParameters) -> Self {
        unimplemented!()
    }
}

impl<'x> From<&'x ChatRoom> for dependency_2::PublicKey {
    fn from(_: &'x ChatRoom) -> Self {
        unimplemented!()
    }
}

// impl<'x> From<&'x ChatRoom> for dependency_2::PublicKey {
//     fn from(_: &'x ChatRoom) -> Self {
//         unimplemented!()
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct ChatRoomSummary;

impl SummaryComponent<dependency_2::ChildSummary> for ChatRoomSummary {
    fn merge(&mut self, _child_summary: dependency_2::ChildSummary) {
        todo!()
    }
}

// #[contract(children(), encoder = BincodeEncoder)]
impl ContractComponent for ChatRoom {
    type Context = String;
    type Parameters = ChatRoomParameters;
    type Delta = String;
    type Summary = ChatRoomSummary;

    fn verify<Child, Ctx>(
        &self,
        parameters: &Self::Parameters,
        _: &Ctx,
        related: &freenet_stdlib::prelude::RelatedContractsContainer,
    ) -> Result<freenet_stdlib::prelude::ValidateResult, freenet_stdlib::prelude::ContractError>
    where
        Child: ContractComponent,
        Self::Context: for<'x> From<&'x Ctx>,
    {
        self.messages
            .verify::<Vec<dependency_1::SignedComposable<dependency_2::ChatRoomMessage>>, _>(
                &parameters.into(),
                self,
                related,
            )?;
        unimplemented!()
    }

    fn verify_delta<Child>(
        _: &Self::Parameters,
        _: &Self::Delta,
    ) -> Result<bool, freenet_stdlib::prelude::ContractError>
    where
        Child: ContractComponent,
    {
        unimplemented!()
    }

    fn merge(
        &mut self,
        _: &Self::Parameters,
        _: &freenet_stdlib::contract_composition::TypedUpdateData<Self>,
        _: &freenet_stdlib::prelude::RelatedContractsContainer,
    ) -> freenet_stdlib::contract_composition::MergeResult {
        unimplemented!()
    }

    fn summarize<ParentSummary>(
        &self,
        _: &Self::Parameters,
        _: &mut ParentSummary,
    ) -> Result<(), freenet_stdlib::prelude::ContractError>
    where
        ParentSummary: freenet_stdlib::contract_composition::SummaryComponent<
            <Self as ContractComponent>::Summary,
        >,
    {
        unimplemented!()
    }

    fn delta(
        &self,
        _: &Self::Parameters,
        _: &Self::Summary,
    ) -> Result<Self::Delta, freenet_stdlib::prelude::ContractError> {
        unimplemented!()
    }
}

pub mod dependency_1 {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Signature {}

    #[derive(Serialize, Deserialize)]
    pub struct SignedComposable<S> {
        pub value: S,
        pub signature: Signature,
    }
}

pub mod dependency_2 {
    use freenet_stdlib::contract_composition::{ContractComponent, ParametersComponent};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub struct ChildSummary;

    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub struct PublicKey {}

    #[derive(Serialize, Deserialize)]
    pub struct ChatRoomMember {
        pub name: String,
        pub public_key: PublicKey,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ChatRoomMessage {
        pub message: String,
        pub author: String,
    }

    pub struct ChatRoomMsgParameters {
        pub owner_public_key: PublicKey,
    }

    impl ParametersComponent for ChatRoomMsgParameters {
        fn contract_id(&self) -> Option<freenet_stdlib::prelude::ContractInstanceId> {
            unimplemented!()
        }
    }

    impl ContractComponent for super::dependency_1::SignedComposable<ChatRoomMessage> {
        type Context = PublicKey;
        type Parameters = ChatRoomMsgParameters;
        type Delta = String;
        type Summary = String;

        fn verify<Child, Ctx>(
            &self,
            parameters: &Self::Parameters,
            context: &Ctx,
            _: &freenet_stdlib::prelude::RelatedContractsContainer,
        ) -> Result<freenet_stdlib::prelude::ValidateResult, freenet_stdlib::prelude::ContractError>
        where
            Child: ContractComponent,
            Self::Context: for<'x> From<&'x Ctx>,
        {
            let _public_key = parameters.owner_public_key;
            let _public_key = PublicKey::from(context);
            // do stuff with pub key
            unimplemented!()
        }

        fn verify_delta<Child>(
            _: &Self::Parameters,
            _: &Self::Delta,
        ) -> Result<bool, freenet_stdlib::prelude::ContractError>
        where
            Child: ContractComponent,
        {
            unimplemented!()
        }

        fn merge(
            &mut self,
            _: &Self::Parameters,
            _: &freenet_stdlib::contract_composition::TypedUpdateData<Self>,
            _: &freenet_stdlib::prelude::RelatedContractsContainer,
        ) -> freenet_stdlib::contract_composition::MergeResult {
            unimplemented!()
        }

        fn summarize<ParentSummary>(
            &self,
            _: &Self::Parameters,
            _: &mut ParentSummary,
        ) -> Result<(), freenet_stdlib::prelude::ContractError>
        where
            ParentSummary: freenet_stdlib::contract_composition::SummaryComponent<
                <Self as ContractComponent>::Summary,
            >,
        {
            unimplemented!()
        }

        fn delta(
            &self,
            _: &Self::Parameters,
            _: &Self::Summary,
        ) -> Result<Self::Delta, freenet_stdlib::prelude::ContractError> {
            unimplemented!()
        }
    }
}