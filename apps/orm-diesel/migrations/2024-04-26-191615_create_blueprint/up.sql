-- Your SQL goes here
create table if not exists public.posts
(
    id        SERIAL PRIMARY KEY,
    title     character varying   not null,
    body      text                not null,
    published boolean             not null default false
);

create table if not exists public.t_blueprint
(
    blueprint_id SERIAL PRIMARY KEY,
    name         character varying(255) not null,                           -- 名称
    description  character varying(255),                                    -- 描述
    user_id      character varying(64)  not null,                           -- 创建用户ID
    config       text                   not null,                           -- 蓝图配置
    state        boolean                not null default true,              -- 蓝图状态
    create_time  timestamp with time zone        default CURRENT_TIMESTAMP, -- 创建时间
    update_time  timestamp with time zone        default CURRENT_TIMESTAMP, -- 更新时间
    is_deleted   boolean                not null default false,             -- 是否删除
    delete_time  timestamp with time zone                                   -- 删除时间
);

comment on column public.t_blueprint.name is '名称';
comment on column public.t_blueprint.description is '描述';
comment on column public.t_blueprint.user_id is '创建用户ID';
comment on column public.t_blueprint.config is '蓝图配置';
comment on column public.t_blueprint.state is '蓝图状态';
comment on column public.t_blueprint.create_time is '创建时间';
comment on column public.t_blueprint.update_time is '更新时间';
comment on column public.t_blueprint.is_deleted is '是否删除';
comment on column public.t_blueprint.delete_time is '删除时间';

