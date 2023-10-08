
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
