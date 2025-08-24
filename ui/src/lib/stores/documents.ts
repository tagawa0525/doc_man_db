import { writable, derived } from 'svelte/store';
import { executeQuery, executeMutation } from '../api/client.js';
import {
  GET_DOCUMENT,
  SEARCH_DOCUMENTS,
  CREATE_DOCUMENT,
  type Document,
  type SearchDocumentsResult,
  type DocumentSearchFilters,
  type CreateDocumentInput,
  type CreatedDocumentWithNumber
} from '../api/queries/documents.js';

// 文書一覧の状態管理
export const documents = writable<Document[]>([]);
export const totalDocuments = writable<number>(0);
export const isLoadingDocuments = writable<boolean>(false);

// 現在選択中の文書
export const currentDocument = writable<Document | null>(null);
export const isLoadingCurrentDocument = writable<boolean>(false);

// 検索フィルター
export const searchFilters = writable<DocumentSearchFilters>({
  title: '',
  documentTypeId: undefined,
  createdBy: undefined,
  createdDateFrom: undefined,
  createdDateTo: undefined,
  limit: 20,
  offset: 0
});

// エラー状態
export const documentsError = writable<string | null>(null);

// 文書検索関数
export async function searchDocuments(filters?: DocumentSearchFilters): Promise<void> {
  console.log('searchDocuments called with filters:', filters);
  isLoadingDocuments.set(true);
  documentsError.set(null);

  try {
    // フィルターをマージ
    let currentFilters: DocumentSearchFilters;
    const unsubscribe = searchFilters.subscribe(value => {
      currentFilters = value;
    });
    unsubscribe();

    const mergedFilters = { ...currentFilters!, ...filters };
    console.log('Executing GraphQL query with merged filters:', mergedFilters);

    const result = await executeQuery<{ searchDocuments: SearchDocumentsResult }>(
      SEARCH_DOCUMENTS,
      { filters: mergedFilters }
    );
    
    console.log('GraphQL query result:', result);

    documents.set(result.searchDocuments.documents);
    totalDocuments.set(result.searchDocuments.total);
  } catch (error: any) {
    console.error('Failed to search documents:', error);
    documentsError.set(error.message || 'Failed to search documents');
    documents.set([]);
    totalDocuments.set(0);
  } finally {
    isLoadingDocuments.set(false);
  }
}

// 文書詳細取得関数
export async function loadDocument(id: number): Promise<void> {
  isLoadingCurrentDocument.set(true);
  documentsError.set(null);

  try {
    const result = await executeQuery<{ document: Document | null }>(
      GET_DOCUMENT,
      { id }
    );

    if (result.document) {
      currentDocument.set(result.document);
    } else {
      throw new Error('Document not found');
    }
  } catch (error: any) {
    console.error('Failed to load document:', error);
    documentsError.set(error.message || 'Failed to load document');
    currentDocument.set(null);
  } finally {
    isLoadingCurrentDocument.set(false);
  }
}

// 文書作成関数
export async function createDocument(input: CreateDocumentInput): Promise<CreatedDocumentWithNumber | null> {
  isLoadingDocuments.set(true);
  documentsError.set(null);

  try {
    const result = await executeMutation<{ createDocument: CreatedDocumentWithNumber }>(
      CREATE_DOCUMENT,
      { input }
    );

    // 作成後、文書一覧を再読み込み
    await searchDocuments();

    return result.createDocument;
  } catch (error: any) {
    console.error('Failed to create document:', error);
    documentsError.set(error.message || 'Failed to create document');
    return null;
  } finally {
    isLoadingDocuments.set(false);
  }
}

// 検索フィルター更新関数
export function updateSearchFilters(newFilters: Partial<DocumentSearchFilters>): void {
  searchFilters.update(current => ({ ...current, ...newFilters }));
}

// ページング関数
export function setPage(page: number, pageSize: number = 20): void {
  const offset = (page - 1) * pageSize;
  updateSearchFilters({ limit: pageSize, offset });
  searchDocuments();
}

// 派生ストア: ページング情報
export const paginationInfo = derived(
  [searchFilters, totalDocuments],
  ([filters, total]) => {
    const limit = filters.limit || 20;
    const offset = filters.offset || 0;
    const currentPage = Math.floor(offset / limit) + 1;
    const totalPages = Math.ceil(total / limit);

    return {
      currentPage,
      totalPages,
      pageSize: limit,
      total,
      hasNextPage: currentPage < totalPages,
      hasPrevPage: currentPage > 1
    };
  }
);

// 初期化関数
export function initializeDocuments(): void {
  console.log('initializeDocuments called');
  // 初期検索を実行
  searchDocuments();
}
