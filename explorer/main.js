methods = [
    "get_chain_name",
    "check_connection",
    "get_contract_list",
    "get_relayer_account_info",
    "get_light_client_header",
    "get_treasury_fungible_token_balance",
    "get_treasury_non_fungible_token_balance"
]

endpoints = {
    "PBC (Postch Beacon Chain)": "http://141.223.124.26:4000/dummy",
    "PBC2 (Postch Beacon Chain)": "http://141.223.124.26:4000/dummy",
}

window.onload = async function(){

    for ([title, url] of Object.entries(endpoints)){
        let info = await fetch_data(url)
        info.title = title
        info.url = url
        console.log(info)
        document.body.innerHTML += generate_card(info)
    }

}

async function fetch_data(url){
    let info = {
        content: {}
    }

    for (method of methods) {

        await fetch(url, {
            method: 'POST',
            headers: {
                'Accept': 'application/json, text/plain, */*',
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({method: method, params: {}})
        })
        .then(res => res.json())
        .then(res => info.content[method] = res)

    }
    return info
}

function generate_card(info) {
    return `
        <div class="card">
            <header class="card-header">
              <p class="card-header-title">
                ${info.title}
                <span class="card-header-url">
                    ${info.url}
                </span>
              </p>
              
            </header>
            <div class="card-content">
              <div class="content">
                <pre>${JSON.stringify(info.content, null, '\t')}</pre>
              </div>
            </div>
        </div>
    `
}
