export interface ContentTypeInterface {
  id: number;
  name: string;
  description: string;
}

export interface ContentTypeDtoInterface {
  name: string;
  description: string;
}

export interface DatabaseInterface {
  id: number;
  datasource_connection_id: number;
  user_id: number;
}

export interface DatabaseDtoInterface {
  datasource_connection_id: number;
  user_id: number;
}


export interface datasourceConnectionInterface {
  id: number;
  datasource_id: number;
  datasource_authentication_type_id?: number;
  connection_name?: string;
  host?: string;
  port?: number;
  username?: string;
  password?: string;
  driver?: string;
  sid?: string;
  url?: string;
  path?: string;
}

export interface datasourceConnectionDtoInterface {
  datasource_id: number;
  datasource_authentication_type_id?: number;
  connection_name?: string;
  host?: string;
  port?: number;
  username?: string;
  password?: string;
  driver?: string;
  sid?: string;
  url?: string;
  path?: string;
}

export interface datasourceInterface {
  id: number;
  type: string;
  description: string;
}

export interface datasourceDtoInterface {
  type: string;
  description: string;
}

export interface datasourceAuthenticationTypeInterface {
  id: number;
  type: string;
  description: string;
}

export interface datasourceAuthenticationTypeDtoInterface {
  type: string;
  description: string;
}

export interface QueryBlockInterface {
  id: number;
  query_file_id: number;
  content_type_id: number;
  serial_order: number;
  query_content_block: string;
}

export interface QueryBlockDtoInterface {
  query_file_id: number;
  content_type_id: number;
  serial_order: number;
  query_content_block: string;
}

export interface QueryFileInterface {
  id: number;
  name: string;
  content: string;
}

export interface QueryFileDtoInterface {
  name: string;
  description: string;
}


export interface UserInterface {
  id: number;
  authorization_level_id: number;
  authentication_type_id: number;
  name: string;
  password: string;
  email: string;
  secondary_email?: string;
}

export interface UserDtoInterface {
  authorization_level_id: number;
  authentication_type_id: number;
  name: string;
  password: string;
  email: string;
  secondary_email?: string;
}

export interface UserCookieInterface {
  id: number;
  name: string;
  email: string;
}

export interface UserAuthenticationTypeInterface  {
    id: number;
    name: string;
    description: string;
}

export interface UserAuthenticationTypeDtoInterface {
  name: string;
  description: string;
}

export interface UserAuthorizationLevelInterface {
  id: number;
  level: number;
  description: string;
}

export interface UserAuthorizationLevelDtoInterface {
  level: number;
  description: string;
}

export interface UserPreviousPasswordInterface {
  id: number;
  user_id: number;
  password: string;
}

export interface UserPreviousPasswordInterface {
  user_id: number;
  password: string;
}
