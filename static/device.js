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