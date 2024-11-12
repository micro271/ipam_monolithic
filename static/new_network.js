
document.getElementById("create_row").addEventListener("click", create_row);

const ID_TBODY = "new_network_body";
const ID_TABLE = "new_network_table";
const ID_CONTAINER = "container_table";
const ID_CONTAINER_BUTTONS_ALL = "div_container_new_network_buttons_all"

function create_row() {
    
    const container = document.getElementById("container_table");
    
    let tbody;
    try {
        tbody = document.getElementById(ID_TBODY);
        if (!tbody) {
            throw new ("tbody doesn't existe");
        }
    } catch {
        const table = document.createElement("table");

        table.classList = "table table-bordered table-hover";
        table.id = ID_TABLE;
        const thead = document.createElement("thead");
        thead.classList = "thead-light";

        const tr = document.createElement("tr");

        const th_n = document.createElement("th");
        th_n.innerHTML = "#";
        th_n.scope = "col";
        tr.appendChild(th_n);    

        const th_network = document.createElement("th");
        th_network.innerHTML = "network";
        th_network.scope = "col";
        tr.appendChild(th_network);


        const th_vlan = document.createElement("th");
        th_vlan.innerHTML = "vlan";
        th_vlan.scope = "col";
        tr.appendChild(th_vlan);


        const th_description = document.createElement("th");
        th_description.innerHTML = "description";
        th_description.scope = "col";
        tr.appendChild(th_description);

        const th_padding = document.createElement("th");
        
        th_padding.colSpan = 2;
        tr.appendChild(th_padding);
        
        thead.appendChild(tr);

        table.appendChild(thead);

        tbody = document.createElement("tbody");
        tbody.id = ID_TBODY;
        table.appendChild(tbody);
        container.append(table);
    }            
    
    //creating row
    const new_row = document.createElement("tr");
    

    const th = document.createElement("th");
    
    const len = tbody.getElementsByTagName("tr").length+1;
    th.scope = "row";
    th.innerHTML = len;
    new_row.appendChild(th);

    if (len > 1) {
        let button_send_all = document.getElementById("new_network_button_send_all");
        if (!button_send_all) {
            button_send_all = document.createElement("button");
            button_send_all.id = "new_network_button_send_all";
            button_send_all.type = "button";
            button_send_all.classList = "btn btn-primary";
            button_send_all.innerHTML = "Send all";

            const button_delete_all = document.createElement("button");
            button_delete_all.id = "new_network_button_remove_all";
            button_delete_all.type = "button";
            button_delete_all.classList = "btn btn-danger ms-2";
            button_delete_all.innerHTML = "Remove all";

            const div = document.createElement("div");
            div.classList = "col";
            div.id = "div_container_new_network_buttons_all";
            div.appendChild(button_send_all);
            div.appendChild(button_delete_all);
            container.appendChild(div);
        }
    }

    //creating input_network
    const input_network = document.createElement("input");
    input_network.type = "text";
    input_network.placeholder = "network";
    input_network.name = "network";

    // input_network.setAttribute("")

    const td_to_input_network = document.createElement("td");
    td_to_input_network.appendChild(input_network);
    new_row.appendChild(td_to_input_network);
    
    //creating input_vlan
    const input_vlan = document.createElement("input");
    input_vlan.type = "number";
    input_vlan.placeholder = "vlan";
    input_vlan.name = "vlan";

    const td_to_input_vlan = document.createElement("td");
    td_to_input_vlan.appendChild(input_vlan);

    new_row.appendChild(td_to_input_vlan);

    // description
    const input_description = document.createElement("input");
    input_description.type = "text";
    input_description.placeholder = "description";
    input_description.name = "description";

    const td_to_input_description = document.createElement("td");
    td_to_input_description.appendChild(input_description);

    new_row.appendChild(td_to_input_description);
    

    //// BUTTON SEND
    const button_send = document.createElement("button");
    button_send.type = "button";
    button_send.classList = "btn btn-primary";
    button_send.innerHTML = "Send";
    button_send.setAttribute('data-row',len);
    button_send.addEventListener("click",send_one);

    const td_to_button_send = document.createElement("td");
    td_to_button_send.classList = "text-center"
    
    td_to_button_send.appendChild(button_send);

    new_row.appendChild(td_to_button_send);
    
    // BUTTON DELETE
    const button_delete = document.createElement("button");
    button_delete.type = "button";
    button_delete.classList = "btn btn-danger";
    button_delete.innerHTML = "RM";
    button_delete.setAttribute('data-row',len);

    button_delete.addEventListener("click", rm_one);

    const td_to_button_delete = document.createElement("td");
    td_to_button_delete.classList = "text-center"
    td_to_button_delete.appendChild(button_delete);

    new_row.appendChild(td_to_button_delete);
    

    tbody.appendChild(new_row);
}


const send_one = (event) => {
    const tg = event.target;
    const row_numner = tg.getAttribute("data-row");
    const row = document.getElementById(ID_TABLE).rows[row_numner];
    const network = row.querySelector('input[name="network"]').value;
    const vlan = row.querySelector('input[name="vlan"]').value;
    const description = row.querySelector('input[name="description"]').value;
    const json = {};

    if (network) {
        json.network = network;
    }

    if (vlan) {
        json.vlan = vlan;
    }

    if (description) {
        json.description = description;
    }
    
    if (json) {
        fetch('/api/network/create',{
            method: 'PUT',
            headers: {
                'Content-type': 'application/json',
            },
            body: JSON.stringify(json)
        }).then(response => {
            if (response.ok) {
                console.log(response)
            }
        });
    }
}

const rm_one = (event) => {
    const tg = event.target;
    const row_number = tg.getAttribute("data-row");

    const table = document.getElementById(ID_TABLE);
    table.rows[row_number].remove();

    reorganize_rows(table);

}

const reorganize_rows = (table) => {
    let rows = table.rows;
    if (rows && rows.length > 0) {
        rows = Array.from(rows).slice(1, table.rows.length);
        for (const [index, row] of rows.entries()) {
            const buttons = row.querySelectorAll('button');
            for (const button of buttons) {
                button.setAttribute('data-row',index+1);
            }
            const th = row.querySelector('th');
            th.innerHTML = index+1;
        }
        if (rows.length == 1) {
            document.getElementById(ID_CONTAINER_BUTTONS_ALL).remove();
        }
    }
}

const send_all_networks = () => {

}