import { gql } from 'graphql-request';

export interface Department {
  id: number;
  code: string;
  name: string;
  parentId?: number;
  level: number;
  managerId?: number;
  description?: string;
  location?: string;
  phoneNumber?: string;
  email?: string;
  budget?: number;
  isActive: boolean;
  createdDate?: string;
  createdAt: string;
  updatedAt: string;
}

export interface DepartmentWithManager {
  id: number;
  code: string;
  name: string;
  parentId?: number;
  parentName?: string;
  level: number;
  managerId?: number;
  managerName?: string;
  description?: string;
  location?: string;
  phoneNumber?: string;
  email?: string;
  budget?: number;
  isActive: boolean;
  employeeCount: number;
  createdDate?: string;
  createdAt: string;
  updatedAt: string;
}

export interface DepartmentSearchFilters {
  name?: string;
  code?: string;
  isActive?: boolean;
  managerName?: string;
  limit?: number;
  offset?: number;
}

export interface CreateDepartmentInput {
  code: string;
  name: string;
  parentId?: number;
  managerId?: number;
  description?: string;
  location?: string;
  phoneNumber?: string;
  email?: string;
  budget?: number;
  createdDate?: string;
}

export interface UpdateDepartmentInput {
  code?: string;
  name?: string;
  parentId?: number;
  managerId?: number;
  description?: string;
  location?: string;
  phoneNumber?: string;
  email?: string;
  budget?: number;
  isActive?: boolean;
}

export const GET_DEPARTMENTS = gql`
  query GetDepartments {
    departments {
      id
      code
      name
      parentId
      parentName
      level
      managerId
      managerName
      description
      location
      phoneNumber
      email
      budget
      isActive
      employeeCount
      createdDate
      createdAt
      updatedAt
    }
  }
`;

export const GET_DEPARTMENT = gql`
  query GetDepartment($id: Int!) {
    department(id: $id) {
      id
      code
      name
      parentId
      parentName
      level
      managerId
      managerName
      description
      location
      phoneNumber
      email
      budget
      isActive
      employeeCount
      createdDate
      createdAt
      updatedAt
    }
  }
`;

export const SEARCH_DEPARTMENTS = gql`
  query SearchDepartments($filters: DepartmentSearchFilters!) {
    searchDepartments(filters: $filters) {
      id
      code
      name
      parentId
      parentName
      level
      managerId
      managerName
      description
      location
      phoneNumber
      email
      budget
      isActive
      employeeCount
      createdDate
      createdAt
      updatedAt
    }
  }
`;

export const CREATE_DEPARTMENT = gql`
  mutation CreateDepartment($input: CreateDepartmentInput!) {
    createDepartment(input: $input) {
      id
      code
      name
      parentId
      level
      managerId
      description
      location
      phoneNumber
      email
      budget
      isActive
      createdDate
      createdAt
      updatedAt
    }
  }
`;

export const UPDATE_DEPARTMENT = gql`
  mutation UpdateDepartment($id: Int!, $input: UpdateDepartmentInput!) {
    updateDepartment(id: $id, input: $input) {
      id
      code
      name
      parentId
      level
      managerId
      description
      location
      phoneNumber
      email
      budget
      isActive
      createdDate
      createdAt
      updatedAt
    }
  }
`;

export const DELETE_DEPARTMENT = gql`
  mutation DeleteDepartment($id: Int!) {
    deleteDepartment(id: $id)
  }
`;
