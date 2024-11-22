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
            console.log(await resp.json());
            if (resp.ok) {
                location.reload(true);
            }
        })
    }
    
}));

[...document.querySelectorAll("[data-ipam-ping]")].forEach(anchor => {
    anchor.addEventListener('click',async (anchor) => {
        const button = anchor.currentTarget;
        const ip = button.getAttribute("data-ipam-ping").replaceAll("_",".");
        const network_id = document.getElementById("network_id").textContent;
        if (ip) {
            const req = await fetch(`/api/device/ping?ip=${ip}&network_id=${network_id}`,{
                method: 'PATCH'
            });
            location.reload(true)
        }
    })
})

document.getElementById("walk").addEventListener("click", async (event) => {
    const button = event.currentTarget;
    if (button.getAttribute("data-ipam-walk") === 'false') {
        button.setAttribute('data-ipam-walk','true');
    } else {
        button.setAttribute('data-ipam-walk','false');
    }

    if (button.getAttribute("data-ipam-walk") === 'true') {
        button.textContent = "Stop"
        button.classList.remove("btn-primary");
        button.classList.add("btn-danger");
        const spin = document.querySelectorAll("[data-ipam-ping]");
        const network_id = document.getElementById("network_id").textContent;
        if (spin) {
            for (const btn of [... spin]) {
                
                btn.style.animation = "spinWalk 0.6s infinite";
                btn.classList.remove("link-danger");
                btn.classList.add("link-success");
                const ip_ = btn.getAttribute("data-ipam-ping");
                const ip = ip_.replaceAll("_",".");
                const ping = await fetch(`/api/device/ping?ip=${ip}&network_id=${network_id}`,{
                    method: 'PATCH'
                });
                const resp = await ping.json();
                console.log(resp);
                const svg = document.getElementById(`svg_${ip_}`);
                if (resp.ping == 'Pong' && !svg.classList.contains('svg-online')) {
                    svg.classList = "svg-online";
                } else if (resp.ping == 'Fail' && !svg.classList.contains('svg-unknown')) {
                    svg.classList = "svg-offline";
                }
                btn.style.animation = "";
                btn.classList.remove("link-success");
                btn.classList.add("link-danger");

                if (button.getAttribute("data-ipam-walk") === 'false') {
                    location.reload(true);
                }
            }
            location.reload(true);
        }
    }    
})