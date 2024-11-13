import {create_row, create_table, send_data} from './main.js'

document.getElementById("new_device_create_all_empty").addEventListener('click', async () => {
    const endpoint = document.getElementById("network_id").innerHTML;
    console.log(endpoint)
    const data = {
        endpoint: `/api/device/all/${endpoint}`,
        headers: {'Content-type': 'application/json'},
        body: null,
        method: 'PUT',
    }
    await send_data(data);
});