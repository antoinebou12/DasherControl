import axios from "axios";

export function get_tenant_token() {
    axios({
        method: 'get',
        url: '/tenants/api/token',
    }).then((response) => {
        return response.data
    })
}

export function createNotification(
    self,
    title,
    description,
    color='dark',
    sticky=true,
    flat=true,
    position="top-right",
    duration=3000,
    buttonClose=true,
    clickClose=true) {
    self.$vs.notification({
        position: position,
        title: title,
        text: description,
        color: color,
        sticky: sticky,
        flat: flat
    })
}