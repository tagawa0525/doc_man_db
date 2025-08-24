// 文書関連のGraphQLミューテーション定義

export { CREATE_DOCUMENT } from '../queries/documents.js';

// 将来的に追加される可能性のあるミューテーション

// 文書更新ミューテーション（プレースホルダー）
export const UPDATE_DOCUMENT = `
  mutation UpdateDocument($id: Int!, $input: UpdateDocumentInput!) {
    updateDocument(id: $id, input: $input) {
      id
      title
      documentTypeId
      createdBy
      createdDate
      createdAt
      updatedAt
    }
  }
`;

// 文書削除ミューテーション（プレースホルダー）
export const DELETE_DOCUMENT = `
  mutation DeleteDocument($id: Int!) {
    deleteDocument(id: $id) {
      success
      message
    }
  }
`;

// TypeScript型定義
export interface UpdateDocumentInput {
  title?: string;
  documentTypeId?: number;
}

export interface DeleteDocumentResponse {
  success: boolean;
  message: string;
}
