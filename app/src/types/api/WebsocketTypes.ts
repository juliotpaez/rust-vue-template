export type WebsocketMessage<T> = WebsocketRequest<T> | WebsocketResponseOrError<T>
export type WebsocketResponseOrError<T> = WebsocketResponse<T> | WebsocketError

export interface WebsocketRequest<T> {
    type: "req",
    id: string,
    method: string,
    params: T,
}

export interface WebsocketNotification<T> {
    type: "not",
    method: string,
    params: T,
}

export interface WebsocketResponse<T> {
    type: "res",
    id: string,
    result: T,
}

export interface WebsocketError {
    type: "err",
    id?: string,
    eid: string,
    message: string,
}

export function isWebsocketError(obj: any): obj is WebsocketError {
    return obj.type === "err";
}

// FS -------------------------------------------------------------------------

export interface ListFilesWsMessage {
    folder: string,
    excludeFiles?: boolean,
    excludeFolders?: boolean
}

// SETTINGS -------------------------------------------------------------------

export interface EditSettingsWsMessage {
    workspace?: string
    viewSettings?: EditViewSettingsWsMessage
}

export interface EditViewSettingsWsMessage {
    projectsGridMode?: boolean
}

// PROJECTS -------------------------------------------------------------------

export interface AddProjectWsMessage {
    name: string
}

export interface EditProjectWsMessage {
    id: number,
    name?: string,
    createdAt?: string,
    notes?: string,
    location?: string
}

export interface ProjectIdWsMessage {
    id: number
}