use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG, MOCK_DATA};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-cadence";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &Config { val: 0 })?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn increment(deps: DepsMut) -> Result<(), ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.val += 1;
    // if increment == 3, lets create large gas consumption scenario to trigger contract jailing
    if config.val == 3 {
        for _ in 0..9 {
            let mut vec = MOCK_DATA.load(deps.storage)?;
            vec.extend(&[0u8; 666666]);
            MOCK_DATA.save(deps.storage, &vec)?;
        }
    }
    CONFIG.save(deps.storage, &config)?;
    Ok(())
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => {
            increment(deps)?;
            Ok(Response::new())
        }
    }
}

// sudo msg
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            increment(deps)?;
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let count = CONFIG.load(deps.storage)?.val;
    Ok(Config { val: count })
}

#[test]
fn test_gas_consumption() -> StdResult<()> {
    let mut deps = mock_dependencies();
    MOCK_DATA.save(&mut deps.storage, &vec![])?;
    CONFIG.save(&mut deps.storage, &Config { val: 2 })?;

    increment(deps.as_mut())?;

    let data = MOCK_DATA.load(&deps.storage)?;
    let byte_count = data.len();

    println!("Byte count: {}", byte_count);

    Ok(())
}
