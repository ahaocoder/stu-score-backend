-- auto-generated definition
create table class_score
(
    id      int auto_increment comment '主键 id'
        primary key,
    stu_num int         not null comment '学号',
    name    varchar(20) not null comment '姓名',
    math    double      null comment '数学成绩',
    english double      null comment '英语',
    chinese double      null comment '语文'
);

create table user
(
    id       int auto_increment comment '主键 id' primary key,
    username varchar(20) not null comment '用户名',
    password varchar(20) not null comment '密码',
    status   tinyint default 0 comment '是否是管理员：0否 1是'
);