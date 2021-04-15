import axios from "axios";

export function get_tenant_token() {
    axios({
        method: 'get',
        url: '/tenants/api/token',
    }).then((response) => {
        return response.data
    })
}