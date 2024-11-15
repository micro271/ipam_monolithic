export {create_table, create_row, send_data};

const create_table = (id_table, id_tbody, container, cols, colSpan) => {
    const table = document.createElement("table");

    table.classList = "table table-bordered table-hover";
    table.id = id_table;
    const thead = document.createElement("thead");
    thead.classList = "thead-light";

    const tr = document.createElement("tr");

    for (const key in cols) {
        const th = document.createElement("th");
        th.innerHTML = cols[key];
        th.scope = "col";        
        if (colSpan) {
            if (colSpan[key]) {
                th.colSpan = colSpan[key];
            }
        }
        tr.appendChild(th);
        
    }
    thead.appendChild(tr);

    table.appendChild(thead);

    const tbody = document.createElement("tbody");
    tbody.id = id_tbody;
    table.appendChild(tbody);
    container.append(table);
    return tbody;
}

const create_row = (cols, table) => {
    
    if(!cols || !table) { return ;}
    const n_rows = table.rows.length;
    const new_row = table.insertRow();
    const th = document.createElement('th');
    th.scope = 'row';
    th.innerText = n_rows+1;
    new_row.appendChild(th);
    for (let i = 1; i <= cols.length; ì++) {
        const cell = new_row.insertCell(i);
        cell.innerHTML = cols[i];
    }

}

const send_data = async (data) => {
    if (!data) { return null ;}
    
    const resp = await fetch(data.endpoint,{
        method: data.method,
        headers: data.headers,
        body: data.body
    });

    return resp
}