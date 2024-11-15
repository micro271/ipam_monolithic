const modifi_network = (event) => {
    const tg = event.target;
    const table = document.getElementById(ID_CURRENT_NETWORKS);
    const row = tg.getAttribute("data-row");
    // TODO
}
const rm_network = (event) => {
    const tg = event.target;
    const table = document.getElementById(ID_CURRENT_NETWORKS);
    const row_number = tg.getAttribute("data-row");
    const row = table.rows[row_number];
    console.log(row.getElementById("network_id"));
}