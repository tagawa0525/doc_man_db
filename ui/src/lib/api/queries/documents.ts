// 文書関連のGraphQLクエリ定義

// 文書詳細取得クエリ
export const GET_DOCUMENT = `
  query GetDocument($id: Int!) {
    document(id: $id) {
      id
      number
      title
      documentTypeId
      createdBy
      createdDate
      createdAt
      updatedAt
    }
  }
`;

// 文書検索クエリ
export const SEARCH_DOCUMENTS = `
  query SearchDocuments($filters: DocumentSearchFilters!) {
    searchDocuments(filters: $filters) {
      documents {
        id
        number
        title
        documentTypeId
        createdBy
        createdDate
        createdAt
        updatedAt
      }
      total
    }
  }
`;

// 文書作成ミューテーション
export const CREATE_DOCUMENT = `
  mutation CreateDocument($input: CreateDocumentInput!) {
    createDocument(input: $input) {
      document {
        id
        number
        title
        documentTypeId
        createdBy
        createdDate
        createdAt
        updatedAt
      }
      documentNumber
      generatedNumber {
        ruleId
        sequenceNumber
        templateUsed
      }
    }
  }
`;

// TypeScript型定義（手動定義、後で自動生成に置き換え）
export interface Document {
  id: number;
  number: string;
  title: string;
  documentTypeId: number;
  createdBy: number;
  createdDate: string;
  createdAt: string;
  updatedAt: string;
}

export interface SearchDocumentsResult {
  documents: Document[];
  total: number;
}

export interface DocumentSearchFilters {
  title?: string;
  documentTypeId?: number;
  createdBy?: number;
  createdDateFrom?: string;
  createdDateTo?: string;
  limit?: number;
  offset?: number;
}

export interface CreateDocumentInput {
  title: string;
  documentTypeCode: string;
  departmentCode: string;
  createdBy: number;
  createdDate: string;
}

export interface GeneratedDocumentNumber {
  ruleId: number;
  sequenceNumber: number;
  templateUsed: string;
}

export interface CreatedDocumentWithNumber {
  document: Document;
  documentNumber: string;
  generatedNumber: GeneratedDocumentNumber;
}
