# Interact 개발 가이드

`interact`는 콜로니 체인들이 공통적으로 가지고 있는 Rust 라이브러리입니다.
`trait ColonyChain`과 다르게 공통적으로 제공해야하는 인터페이스가 있거나 콜로니 체인에게 일괄적으로 적용되어야 하는 내용이 있는 것은 아니고,
다만 통상적으로 콜로니 체인을 다룰 때에 필요할 것으로 예상되는 일종의 태스크이자 가이드입니다.

`tests/suite1.rs`는 `interact`라이브러리의 핵심 기능을 테스트하는 코드입니다.
대략적으로 `interact`가 어떤 기능을 제공해야 하는 지에 대한 아웃라인이기도 합니다. (TDD)

## Methods

대략적으로 다음과 같은 메소드를 제공해야 합니다.

```rust
async fn get_current_height() -> Result<u64, Error>;
async fn get_block(height: u64) -> Result<Block, Error>;
async fn query_account(addr: Address) -> Result<Account, Error>;
async fn query_contract_state(contract_addr: Address, field_name: &str) -> Result<SomeGenericTypeLikeJson, Error>;
async fn transfer_native_token(signer_info: SignerInfo, receiver_public_key: Address,
which_token: SomeIdentifier, amount: rust_decimal::Decimal, tx) -> Result<TxId, Error>;
async fn execute_contract_method(signer_info: SignerInfo, contract_addr: Address, method_name: &str, arguments: SomeGenericTypeLikeJson)
 -> Result<SomeGenericTypeLikeJson, Error>;
```

1. `Address`, `Block,` `Account`, `SomeGenericTypeLikeJson`, `Error` 등의 타입들은 임의로 잘 설정하시면 됩니다.
1. 모든 함수에 공통적으로 `fullnode_url`을 받아주세요. (보기 편하기 위해 위 코드에선 생략했습니다)

## trait ColonyChain

`trait ColonyChain`은 위 함수들을 적절히 사용해서 구현하면 됩니다.

## Typescript Integration

어떤 열악한 콜로니 체인들은 해당 블록체인과 상호작용하기 위해 사용하는 클라이언트 사이드 러스트 라이브러리가 없거나 매우 빈약할 수도 있습니다.
대부분의 경우에는 API가 JSON-RPC등으로 제공 될 것이기 때문에 직접 HTTP 요청을 인코딩 해서 날리는 것으로 해결 할 수 있으나 프로토콜이 너무 복잡하거나 하면
(제일 지원이 잘 되어있을 확률이 높은) 자바스크립트 라이브러리를 사용해야할 수도 있습니다.

이 경우 다음과 같은 스텝을 따르시면 됩니다.

1. 위에 있는 `interact` 에 들어있는 Rust 메소드에 일대일 대응되는 타입스크립트 함수들을 작성합니다.
2. 해당 함수의 이름을 path로, 파라미터를 JSON content로 인식해서 핸들하고
결과값도 JSON으로 돌려주는 간단한 HTTP 서버를 만듭니다 (이를 API 서비스라고 부릅니다).
3. `interact`의 구현체는 해당 API 서비스가 이미 로컬에 띄워져있다고 가정하고 HTTP 요청을 보냅니다.
