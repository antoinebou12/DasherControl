import axios from "axios";

export default {
    get_tenant_token() {
        axios({
            method: 'get',
            url: '/tenants/api/token',
        }).then((response) => {
            return response.data
        })
    }
}