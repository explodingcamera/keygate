import { createClient, type NormalizeOAS } from "fets";

import type publicAPI from "./../../api/public";
import type privateAPI from "./../../api/private";

const publicClient = createClient<NormalizeOAS<typeof publicAPI>>({});
const privateClient = createClient<NormalizeOAS<typeof privateAPI>>({});

export { publicClient, privateClient };
