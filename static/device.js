import {send_data} from './main.js'

const  button_create_all = document.getElementById("new_device_create_all_empty");

if (button_create_all) {

    button_create_all.addEventListener('click', async () => {
        const endpoint = document.getElementById("network_id").innerHTML;
        console.log(endpoint)
        const resp = await fetch(`/api/v1/device/${endpoint}`,{
            headers: {"Content-type": "application/json"},
            method: 'POST',
        });
        
        if (resp.ok) {
            location.reload(true)
        }
    });
}


const button_new_subnet = document.getElementById("subnet");

if (button_new_subnet) {
    subnet.addEventListener('click',() => {
        const modal = document.getElementById("subnetting");
        const network = document.getElementById("network").textContent.split('/')[0];
        const network_id = document.getElementById("network_id").textContent;
        const input_network = modal.querySelector("[name='network']");
        input_network.value = network;
        new bootstrap.Modal(modal).show();

        modal.querySelector(".save").addEventListener('click',async () => {
            const prefix = modal.querySelector("[name='prefix']").value;
            const resp = await fetch(`/api/v1/network/subnet?father_id=${network_id}&prefix=${Number(prefix)}`, {
                method: 'POST'
            });
            if (resp.ok) {
                location.reload(true);
            }
        });
    });
}

const table_main_network_chiled = document.getElementById("table_main");

if (table_main_network_chiled) {
    const button_modify = table_main_network_chiled.querySelectorAll("[data-type-button='modify']");
    [... button_modify].map(btn => {

        btn.addEventListener("click", event => {
            const table = document.getElementById("table_main");
            const btn = event.currentTarget;
            const row_number = btn.getAttribute("data-row");
            const row = table.rows[row_number];
            

            const id = row.querySelector("[data-name='id']").textContent;
            const network = row.querySelector("[data-name='network'] a");
            const vlan = row.querySelector("[data-name='vlan']");
            const description = row.querySelector("[data-name='description']");
            const avl = row.querySelector("[data-name='available']");
            const used = row.querySelector("[data-name='used']");
            const free = row.querySelector("[data-name='free']");

            const modal = document.querySelector(".modal");
            const id_input = modal.querySelector("[name='id']");
            const network_input = modal.querySelector("[name='network']");
            const vlan_input = modal.querySelector("[name='vlan']");
            const description_input = modal.querySelector("[name='description']");
            const avl_input = modal.querySelector("[name='available']");
            const used_input = modal.querySelector("[name='used']");
            const free_input = modal.querySelector("[name='free']");
            
            id_input.value = id;
            network_input.value = network.textContent;
            vlan_input.value = vlan.textContent;
            description_input.value = description.textContent;
            avl_input.value = avl.textContent;
            used_input.value = used.textContent;
            free_input.value = free.textContent;

            new bootstrap.Modal(modal).show();

            const checkbox = modal.querySelector("#checkbox_network_to_change");

            checkbox.addEventListener('change',() => {
                if(checkbox.checked) {
                    network_input.disabled = false;
                } else {
                    network_input.disabled = true;
                }
            });

            ////cambiar
            const modal_save_event = async () => {
                const vlan_input = modal.querySelector("[name='vlan']");
                const description_input = modal.querySelector("[name='description']");
                const network_input = modal.querySelector("[name='network']");
                
                const to_send = {};
                if (vlan_input.value && vlan_input.value != vlan.textContent) {
                    to_send.vlan = vlan_input.value;
                }
                if (description_input.value && description_input.value != description.textContent) {
                    to_send.description = description_input.value;
                }
                if (checkbox.checked && network_input != network.textContent) {
                    to_send.network = network_input.value;
                }

                const resp = await fetch(`/api/v1/network/${id}`, {
                    method: 'PATCH',
                    body: JSON.stringify(to_send),
                    headers: {"Content-type": 'application/json'}
                });

                const resp_json = await resp.json();
                if (resp_json.status == 200) {
                    if(to_send.network) {
                        network.textContent = to_send.network;
                    }
                    if(to_send.description) {
                        description.textContent = to_send.description;
                    }
                    if(to_send.vlan) {
                        vlan.textContent = to_send.vlan;
                    }
                }

                bootstrap.Modal.getInstance(modal).hide();
            }
        
            modal.querySelector(`.save`).addEventListener('click',modal_save_event);
            modal.addEventListener('hidden.bs.modal',() => modal.querySelector(`.save`).removeEventListener('click',modal_save_event));
        })
    });
}