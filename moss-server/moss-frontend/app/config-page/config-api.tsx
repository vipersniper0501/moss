
export type MossData = {
    server: string;
    approved_files: MossFileData[];
    invalid_files: MossFileData[];
    valid_users: string[];
    invalid_users: string[];
};

export type MossFileData = {
    name: string;
    location: string;
};

export function jsonToMossData(data: string | MossData): MossData | string {

    if (data == undefined) {
        console.log("data is undefined");
        return "No data";
    }
    if (typeof data === "object") {
        console.log("Data is an object. Returning.")
        return data;
    }
    if (data == "No data") {
        return data;
    }
    const jsonObject: MossData = JSON.parse(data);
    var app_files: MossFileData[] = [];
    var inv_files: MossFileData[] = [];

    jsonObject.approved_files.forEach( (val) => {
            const mfd: MossFileData = {
                name: val.name,
                location: val.location
            }
            app_files.push(mfd);
    });

    jsonObject.invalid_files.forEach( (val) => {
            const mfd: MossFileData = {
                name: val.name,
                location: val.location
            }
            inv_files.push(mfd);
    });

    const mossdata: MossData = {
        server: jsonObject.server,
        approved_files: app_files,
        invalid_files: inv_files,
        valid_users: jsonObject.valid_users,
        invalid_users: jsonObject.invalid_users

    }

    return mossdata;
    

}
