import {send_data} from './main.js'

const popoverTriggerList = document.querySelectorAll('[data-bs-toggle="popover"]')
const popoverList = [...popoverTriggerList].map(popoverTriggerEl => {
    new bootstrap.Popover(popoverTriggerEl,{html: true})
    
});


const reserved_button = document.getElementById('reserve_ip');

if (reserved_button) {
    reserved_button.addEventListener('click', reserve_ip);
}

const reserve_ip = (event) => {
    console.log(event.target.parentNode)
}


[...popoverTriggerList].map(popOver => popOver.addEventListener('shown.bs.popover', () => {
    const pop_over = bootstrap.Popover.getInstance(popOver)
    const id_father = pop_over.tip.id;
    const father = document.getElementById(id_father);

    const status = father.querySelector("#data_status");
    const ip = father.querySelector('.popover-header').textContent;
    const ip_ = ip.replaceAll('.','_');
    const button_reserved = document.getElementById(ip_);    
    const network_id = document.getElementById('network_id').textContent;

    if (button_reserved){
        button_reserved.addEventListener('click', async (event) => {
            const data = {
                endpoint: `/api/device/one?ip=${ip}&network_id=${network_id}`,
                method: 'PATCH',
                body: JSON.stringify({
                    status: 'Reserved'
                }),
                headers: {
                    'Content-type': 'application/json'
                }
            }
            const resp = await send_data(data);
            if (resp.ok) {
                location.reload(true)
            }
        })
    }
    
}));