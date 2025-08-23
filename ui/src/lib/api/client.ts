import { GraphQLClient } from 'graphql-request';
import { browser } from '$app/environment';

// GraphQL エンドポイント設定
const GRAPHQL_ENDPOINT = browser
  ? 'http://localhost:8080/graphql'  // ブラウザから
  : 'http://localhost:8080/graphql'; // SSR時

// GraphQLクライアント初期化
export const graphqlClient = new GraphQLClient(GRAPHQL_ENDPOINT, {
  headers: {
    'Content-Type': 'application/json',
  },
});

// リクエストインターセプター（認証等に使用）
export function setAuthToken(token: string) {
  graphqlClient.setHeader('Authorization', `Bearer ${token}`);
}

// エラーハンドリング用のラッパー関数
export async function executeQuery<T>(query: string, variables?: any): Promise<T> {
  try {
    return await graphqlClient.request<T>(query, variables);
  } catch (error: any) {
    console.error('GraphQL Query Error:', error);

    // GraphQLエラーの詳細を抽出
    if (error.response?.errors) {
      const graphqlErrors = error.response.errors.map((err: any) => err.message).join(', ');
      throw new Error(`GraphQL Error: ${graphqlErrors}`);
    }

    // ネットワークエラー等
    if (error.response?.status) {
      throw new Error(`HTTP Error ${error.response.status}: ${error.response.statusText}`);
    }

    // その他のエラー
    throw new Error(error.message || 'Unknown error occurred');
  }
}

// ミューテーション用のラッパー関数
export async function executeMutation<T>(mutation: string, variables?: any): Promise<T> {
  return executeQuery<T>(mutation, variables);
}

// ヘルスチェック用の関数
export async function checkApiHealth(): Promise<boolean> {
  try {
    const healthQuery = `
      query {
        __typename
      }
    `;
    await executeQuery(healthQuery);
    return true;
  } catch (error) {
    console.warn('API Health Check Failed:', error);
    return false;
  }
}
