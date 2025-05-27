export const getBasePath = (): string => {
    return process.env.PUBLIC_URL || "";
};

export const getRomPath = (relativePath: string): string => {
    const basePath = getBasePath();
    return `${basePath}/roms/${relativePath}`;
};
