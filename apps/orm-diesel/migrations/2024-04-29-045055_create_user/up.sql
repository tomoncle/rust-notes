/*
 * MIT License
 *
 * Copyright (c) 2023 tomoncle
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

-- Your SQL goes here

create table if not exists public.t_user
(
    user_id     SERIAL PRIMARY KEY,                                        -- 用户ID
    name        character varying(255) not null,                           -- 名称
    description character varying(255),                                    -- 描述
    config      text                   not null,                           -- 用户配置
    state       boolean                not null default true,              -- 用户状态
    create_time timestamp with time zone        default CURRENT_TIMESTAMP, -- 创建时间
    update_time timestamp with time zone        default CURRENT_TIMESTAMP, -- 更新时间
    is_deleted  boolean                not null default false,             -- 是否删除
    delete_time timestamp with time zone                                   -- 删除时间
);

comment on column public.t_user.name is '名称';
comment on column public.t_user.description is '描述';
comment on column public.t_user.user_id is '创建用户ID';
comment on column public.t_user.config is '用户配置';
comment on column public.t_user.state is '用户状态';
comment on column public.t_user.create_time is '创建时间';
comment on column public.t_user.update_time is '更新时间';
comment on column public.t_user.is_deleted is '是否删除';
comment on column public.t_user.delete_time is '删除时间';
