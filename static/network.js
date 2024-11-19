


const modal = document.querySelector('.modal')
if (modal) {
    modal.addEventListener('show.bs.modal', event => {
        // Button that triggered the modal
        const button = event.relatedTarget;
        const row_number = button.getAttribute("data-row");
        const table = document.getElementById("table_main");
        
        if (table) {
            const row = table.rows[row_number];

            const id = row.querySelector("[data-name='id']");
            const network = row.querySelector("[data-name='network']");
            const vlan = row.querySelector("[data-name='vlan']");
            const desc = row.querySelector("[data-name='description']");
            const avl = row.querySelector("[data-name='available']");
            const used = row.querySelector("[data-name='used']");
            const free = row.querySelector("[data-name='free']");
            
            const inputs = modal.querySelector(".modal-body");
            
            const input_id = inputs.querySelector("[data-input-name='id']");
            input_id.value = id.textContent;

            const input_network = inputs.querySelector("[data-input-name='network']");
            input_network.value = network.textContent;
            const input_vlan = inputs.querySelector("[data-input-name='vlan']");
            input_vlan.value = vlan.textContent;
            const input_desc = inputs.querySelector("[data-input-name='description']");
            input_desc.value = desc.textContent;
            const input_avl = inputs.querySelector("[data-input-name='available']");
            input_avl.value = avl.textContent;
            const input_used = inputs.querySelector("[data-input-name='used']");
            input_used.value = used.textContent;
            const input_free = inputs.querySelector("[data-input-name='free']");
            input_free.value = free.textContent;
        }
        
        modal.querySelector("#checkbox_network_to_change").addEventListener("change", (event) => {
            const tmp = event.target;
            const network_input = modal.querySelector("[data-input-name='network']");
            
            if (tmp.checked && network_input) {
                network_input.disabled = false;
            } else if (!tmp.checked) {
                network_input.disabled = true;
            }
        }, {once: true});

        const button_seave = modal.querySelector(".save");

        button_seave.addEventListener('click',async () => {
            const input_network = modal.querySelector("[data-input-name='network']").value; 
            const input_vlan = modal.querySelector("[data-input-name='vlan']").value;
            const input_desc = modal.querySelector("[data-input-name='description']").value;
            const id = modal.querySelector("[data-input-name='id']").value;
            const checked_edit_network = modal.querySelector("#checkbox_network_to_change");
            const new_data = {};

            if (checked_edit_network.checked) {
                new_data.network = input_network;
            }

            if (input_vlan) {
                new_data.vlan = input_vlan;
            }
            if(input_desc) {
                new_data.description = input_desc;
            }
            
            if (Object.keys(new_data).length > 0) {
                const resp = await fetch(`/api/network/${id}`,{
                    headers: {'Content-type':'application/json'},
                    method: 'PATCH',
                    body: JSON.stringify(new_data),
                });

                const resp_data = await resp.json();
                if (resp_data.status === 200) {
                    console.log(resp_data);
                } else {
                    console.log(resp_data);
                }
            }
        })
    },
    {once: true})
}
