variable "aws_region" {
  default = "us-east-1"
}

variable "auth_postgres_db" {
  type      = string
  sensitive = true
}

variable "auth_postgres_password" {
  type      = string
  sensitive = true
}

variable "auth_postgres_user" {
  type      = string
  sensitive = true
}

variable "auth_postgres_port" {
  type      = string
  sensitive = true
}

variable "auth_postgres_host" {
  type      = string
  sensitive = true
}

variable "auth_database_url" {
  type      = string
  sensitive = true
}

variable "task_postgres_db" {
  type      = string
  sensitive = true
}

variable "task_postgres_password" {
  type      = string
  sensitive = true
}

variable "task_postgres_user" {
  type      = string
  sensitive = true
}

variable "task_postgres_port" {
  type      = string
  sensitive = true
}

variable "task_postgres_host" {
  type      = string
  sensitive = true
}

variable "task_database_url" {
  type      = string
  sensitive = true
}

variable "auth_url" {
  type      = string
  sensitive = true
}

variable "redis_url" {
  type      = string
  sensitive = true
}