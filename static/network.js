import { create_table, send_data } from "/static/main.js";

document.getElementById("create_row").addEventListener("click", new_network);
const ID_TBODY = "new_network_body";
const ID_NEW_NETWORK_TABLE = "new_network_table";
const ID_CONTAINER_TABLE_NEW_NETWORK = "container_table";
const ID_CONTAINER_BUTTONS_ALL = "div_container_new_network_buttons_all"
const ID_TABLE_CURRENT_NETWORKS = "table_main";
const table_main_pointer = document.getElementById(ID_TABLE_CURRENT_NETWORKS);

let modal;

function new_network() {
    
    const container = document.getElementById("container_table");
    
    let tbody;
    try {
        tbody = document.getElementById(ID_TBODY);
        if (!tbody) {
            throw new ("tbody doesn't existe");
        }
    } catch {
        const cols = {
            1: "#",
            2: "network",
            3: "vlan",
            4: "description",
            5: "",
        };
        const colSpan = {
            5: 2,
        };
        tbody = create_table(ID_NEW_NETWORK_TABLE, ID_TBODY, container, cols, colSpan);
    }            
    //creating row
    const new_row = document.createElement("tr");
    new_row.style.animation = "slideNetwork 0.2s ease forwards";
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

            button_send_all.addEventListener('click', send_all_networks);

            const button_delete_all = document.createElement("button");
            button_delete_all.id = "new_network_button_remove_all";
            button_delete_all.type = "button";
            button_delete_all.classList = "btn btn-danger ms-2";
            button_delete_all.innerHTML = "Remove all";

            button_delete_all.addEventListener('click', (event) => {
                const table = document.getElementById(ID_NEW_NETWORK_TABLE);
                if (table) { table.remove(); }
                const div = document.getElementById(ID_CONTAINER_BUTTONS_ALL);
                div.remove();
            });

            const div = document.createElement("div");
            div.classList = "col";
            div.id = ID_CONTAINER_BUTTONS_ALL;
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

const send_one = async (event) => {
    const tg = event.target;
    const row_numner = tg.getAttribute("data-row");
    const table = document.getElementById(ID_NEW_NETWORK_TABLE);
    const row = table.rows[row_numner];
    const json = get_data_network_to_send(row);
    
    if (json) {
        const data = {
            body: JSON.stringify(json),
            method: 'PUT',
            endpoint: '/api/network/create',
            headers: {'Content-type': 'application/json'}
        }
        const resp = await send_data(data);
        const resp_data = await resp.json();
        
        if (resp_data.status === 201) {
            row.remove();
            if (table.rows.length == 1) {
                table.remove()
                location.reload()
            } else if (table.rows.length == 2) {
                const btn  =document.getElementById(ID_CONTAINER_BUTTONS_ALL);
                if (btn) {
                    btn.remove()
                }
            }
            reorganize_rows(table)
            if (!document.getElementById(ID_TABLE_CURRENT_NETWORKS)) {
                create_table_main_if_not_exists()
            }
            add_row_table_main(resp_data.data)
            // TODO! add_row_table_main() When we send the data and the response is OK, if the table has some rows, we append the new data to the main table
        } else {
            console.log(resp_data)
        }
    }
}

const get_data_network_to_send = (row) => {
    const json = {}
    const network = row.querySelector('input[name="network"]').value;
    const vlan = row.querySelector('input[name="vlan"]').value;
    const description = row.querySelector('input[name="description"]').value;
    
    if (network) {
        json.network = network;
    }
    if (vlan) {
        json.vlan = vlan;
    }
    if (description) {
        json.description = description;
    }
    if (Object.keys(json).length == 0) {
        return null;
    }
    return json;
}


const rm_one = (event) => {
    const tg = event.target;
    const row_number = tg.getAttribute("data-row");
    const table = document.getElementById(ID_NEW_NETWORK_TABLE);
    table.deleteRow(row_number);
    if (table.rows.length == 2) {
        document.getElementById(ID_CONTAINER_BUTTONS_ALL).remove();
    }
    reorganize_rows(table);
}

const reorganize_rows = (table) => {
    const rows = Array.from(table.rows).slice(1);
    if (rows.length == 0) {
        table.remove();
    } else {
        for (const [index, row] of rows.entries()) {
            const buttons = row.querySelectorAll('button');
            for (const button of buttons) {
                button.setAttribute('data-row',index+1);
            }
            const th = row.querySelector('th');
            th.innerHTML = index+1;
        }
    }
}

const send_all_networks = async () => {
    const table_new_network = document.getElementById(ID_NEW_NETWORK_TABLE);

    if (table_new_network) {
        const rows = Array.from(table_new_network.rows).slice(1);
        if (rows) {
            for (const row of rows) {
                const data = get_data_network_to_send(row);
                if (data){
                    const send = await fetch('/api/network/create',{
                        method: 'PUT',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify(data)
                    });
                    const data_resp = await send.json();
                    
                    if (data_resp.status === 201) {
                        row.remove();
                        reorganize_rows(document.getElementById(ID_NEW_NETWORK_TABLE));
                        if (table_new_network.rows.length == 2) {
                            const div_container_buttons = document.getElementById("div_container_new_network_buttons_all");
                            if (div_container_buttons){
                                div_container_buttons.remove();
                            }
                        }
                        if (!document.getElementById(ID_TABLE_CURRENT_NETWORKS)) {
                            create_table_main_if_not_exists()
                        }
                        add_row_table_main(data_resp.data)
                    }                
                }
    
            }
        }
        if (table_new_network.rows.length <= 1) {
            location.reload(true);
        }
    }
}

const rm_network = async (event) => {
    const tg = event.target;
    const row_number = tg.getAttribute("data-row");
    const table = document.getElementById(ID_TABLE_CURRENT_NETWORKS);
    const row = table.rows[row_number];
    const network_id = row.cells[1].textContent;
    const resp = await fetch(`/api/network/${network_id}`,{
        method: 'DELETE'
    });
    if (resp.ok) {
        row.remove()
        reorganize_rows(table)
    }
}

const modal_event = (event) => {
    
    const button = event.target;
    const row_number = button.getAttribute("data-row");
    const table = document.getElementById("table_main");
    const modal = document.getElementById("modifNetworkModal");

    const row = table.rows[row_number];
    const id = row.querySelector("[data-name='id']");
    const id_value = id.textContent;
    const network = row.querySelector("[data-name='network']");
    const actual_value_network = network.textContent;
    const vlan = row.querySelector("[data-name='vlan']");
    const actual_value_vlan = vlan.textContent;
    const desc = row.querySelector("[data-name='description']");
    const actual_value_desc = desc.textContent;


    const avl = row.querySelector("[data-name='available']");
    const used = row.querySelector("[data-name='used']");
    const free = row.querySelector("[data-name='free']");
    
    const inputs = modal.querySelector(".modal-body");
    
    const input_id = inputs.querySelector("[data-input-name='id']");
    input_id.value = id_value;

    const input_network = inputs.querySelector("[data-input-name='network']");
    input_network.value = actual_value_network;
    const input_vlan = inputs.querySelector("[data-input-name='vlan']");
    input_vlan.value = actual_value_vlan;
    const input_desc = inputs.querySelector("[data-input-name='description']");
    input_desc.value = actual_value_desc;
    const input_avl = inputs.querySelector("[data-input-name='available']");
    input_avl.value = avl.textContent;
    const input_used = inputs.querySelector("[data-input-name='used']");
    input_used.value = used.textContent;
    const input_free = inputs.querySelector("[data-input-name='free']");
    input_free.value = free.textContent;
    
    modal.querySelector("#checkbox_network_to_change").addEventListener("change", (event) => {
        const tmp = event.target;
        const network_input = modal.querySelector("[data-input-name='network']");
        
        if (tmp.checked && network_input) {
            network_input.disabled = false;
        } else if (!tmp.checked) {
            network_input.disabled = true;
        }
    }); 
    
    new bootstrap.Modal(modal).show();

    const modal_save_event = async () => {
        const input_network = modal.querySelector("[data-input-name='network']").value; 
        const input_vlan = modal.querySelector("[data-input-name='vlan']").value;
        const input_desc = modal.querySelector("[data-input-name='description']").value;
        const checked_edit_network = modal.querySelector("#checkbox_network_to_change");
        const new_data = {};
    
    
        if (checked_edit_network.checked && input_network != actual_value_network) {
            new_data.network = input_network;
        }
        if (input_vlan != actual_value_vlan) {
            new_data.vlan = input_vlan;
        }
        if(input_desc != actual_value_desc) {
            new_data.description = input_desc;
        }
        
        
        if (Object.keys(new_data).length > 0) {
            const resp = await fetch(`/api/network/${id_value}`,{
                headers: {'Content-type':'application/json'},
                method: 'PATCH',
                body: JSON.stringify(new_data),
            });
            const resp_data = await resp.json();
            if (resp_data.status === 200) {
                if (table_main_pointer) {
                    const row = table_main_pointer.rows[row_number];
                    if (row) {
                        if (new_data.network) {
                            row.querySelector("[data-name='network']").textContent = new_data.network;
                        }
                        if (new_data.vlan) {
                            row.querySelector("[data-name='vlan']").textContent = new_data.vlan;
                        }
                        if (new_data.description) {
                            row.querySelector("[data-name='description']").textContent = new_data.description;
                        }
                    }
                    bootstrap.Modal.getInstance(modal).hide();
                }
            } else {
                console.log(resp_data);
            }
        }
    }

    modal.querySelector(`.save`).addEventListener('click',modal_save_event);
    modal.addEventListener('hidden.bs.modal',() => modal.querySelector(`.save`).removeEventListener('click',modal_save_event));
    
}

if (table_main_pointer) {
    const buttons_rm = table_main_pointer.querySelectorAll('[data-type-button="rm"]');
    const button_modify = table_main_pointer.querySelectorAll('[data-type-button="modify"]');
    Array.from(buttons_rm).forEach(button => {
        button.addEventListener('click', rm_network);
    });

    [...button_modify].forEach(btn => {
        btn.addEventListener('click', modal_event);
    })
}
const add_row_table_main = (rows) => {
    const table = document.getElementById(ID_TABLE_CURRENT_NETWORKS);
    if (rows && table) {
        const body = table.querySelector("tbody");
        
        for (const row of Array.from(rows)) {
            const len = table.rows.length;
            const {id, vlan, network, description, available, used, free} = row;
            const new_row = body.insertRow();
            new_row.style.opacity = '1';
            new_row.style.animation = "slideNetwork 0.5s ease forwards";
            const th = document.createElement('th');
            th.textContent = len;
            th.scope = "row";
            th.classList = 'd-none d-lg-table-cell';

            new_row.appendChild(th);
            const td_id = new_row.insertCell();
            td_id.classList = 'd-none d-lg-table-cell';
            td_id.setAttribute("data-name","id");
            td_id.textContent= id;

            const td_network = new_row.insertCell();
            td_network.setAttribute("data-name","network");
            td_network.innerHTML = network;

            const td_vlan = new_row.insertCell();
            td_vlan.setAttribute("data-name","vlan");
            if (vlan) {
                td_vlan.textContent= vlan;
            }

            const td_desc = new_row.insertCell();
            td_desc.setAttribute("data-name","description");
            td_desc.textContent= description;

            const td_avl = new_row.insertCell();
            td_avl.textContent = available;
            td_avl.classList = "d-none d-lg-table-cell";
            td_avl.setAttribute("data-name","available");

            const td_used = new_row.insertCell();
            td_used.setAttribute("data-name","used");
            td_used.textContent= used;

            const td_free = new_row.insertCell();
            td_free.setAttribute("data-name","free");
            td_free.textContent= free;

            const td_button_device = new_row.insertCell();
            const anchor_device = document.createElement('a');
            anchor_device.href = `/devices/${id}`;
            anchor_device.textContent = 'Devices';
            td_button_device.appendChild(anchor_device);

            
            const td_button_modify = new_row.insertCell();
            const button_modify = document.createElement('button');
            button_modify.textContent = "Modify";
            button_modify.addEventListener('click',modal_event);
            button_modify.type = 'button';
            button_modify.classList = 'btn btn-primary';
            button_modify.setAttribute('data-type-button','modify');
            button_modify.setAttribute('data-row', len);
            button_modify.setAttribute('data-bs-toggle', "modal");
            button_modify.setAttribute('data-bs-target', "#modifNetworkModal");
            td_button_modify.appendChild(button_modify);


            const td_button_rm = new_row.insertCell();
            const button_rm = document.createElement('button');
            button_rm.textContent = "RM";
            button_rm.type = 'button';
            button_rm.classList = 'btn btn-danger';
            button_rm.setAttribute('data-type-button','rm');
            button_rm.setAttribute('data-row', len);
            button_rm.addEventListener('click',rm_network);
            td_button_rm.appendChild(button_rm);           
        }
    }
}

const create_table_main_if_not_exists = () => {
    const table = document.createElement("table");
    const container = document.getElementById("container");
    table.classList = "table table-hover text-center align-middle";
    table.id = ID_TABLE_CURRENT_NETWORKS;
    const thead = document.createElement("thead");

    const tr = document.createElement("tr");

    const th_num = document.createElement("th");
    th_num.scope = "col";
    th_num.textContent = "#";
    th_num.classList = "d-none d-lg-table-cell";
    tr.appendChild(th_num);


    const th_id = document.createElement("th");
    th_id.scope = "col";
    th_id.textContent = "id";
    th_id.classList = "d-none d-lg-table-cell";
    tr.appendChild(th_id);

    const th_network = document.createElement("th");
    th_network.scope = "col";
    th_network.textContent = "network";
    tr.appendChild(th_network);

    const th_vlan = document.createElement("th");
    th_vlan.scope = "col";
    th_vlan.textContent = "vlan";
    tr.appendChild(th_vlan);

    const th_desc = document.createElement("th");
    th_desc.scope = "col";
    th_desc.textContent = "description";
    tr.appendChild(th_desc);

    const th_avl = document.createElement("th");
    th_avl.scope = "col";
    th_avl.classList = "d-none d-lg-table-cell";
    th_avl.textContent = "available";
    tr.appendChild(th_avl);

    const th_used = document.createElement("th");
    th_used.scope = "col";
    th_used.textContent = "used";
    tr.appendChild(th_used);

    const th_free = document.createElement("th");
    th_free.scope = "col";
    th_free.textContent = "free";
    tr.appendChild(th_free);

    const th_empty = document.createElement("th");
    if (role === "Admin") {
        th_empty.colSpan = 3;
    } else {
        th_empty.colSpan = 1;
    }
    tr.appendChild(th_empty);

    
    thead.appendChild(tr);

    table.appendChild(thead);

    const tbody = document.createElement("tbody");
    tbody.classList = "table-group-divider";
    tbody.id = "table_main_tbody";
    table.appendChild(tbody);
    if (container.firstChild) {
        container.insertBefore(table, container.firstChild);
    } else {
        container.appendChild(table);
    }
}

