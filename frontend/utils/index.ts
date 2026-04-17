import axios from "axios";

export { createNotification } from "./notify";

export function get_tenant_token() {
  axios({
    method: "get",
    url: "/tenants/api/token",
  }).then((response) => {
    return response.data;
  });
}
