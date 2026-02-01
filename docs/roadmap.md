# Tickly 0.2.0 ~ 0.6.0 상세 스펙 초안

이 문서는 0.2.0부터 0.6.0까지의 MVP 범위와 화면/데이터 변경을 요약합니다.

## 공통 방향
- **일일 자동 초기화는 제거**하고, 반복 규칙으로 대체합니다.
- 성과/통계는 **완료율 계산 없이** 스트릭(연속 달성)만 제공합니다.
- 알림 기능은 Tauri 충돌 이슈로 보류합니다.

## 0.2.0 — 반복 규칙 스케줄링 ✅ 완료
**목표:** 일일 초기화 제거 후, 반복 규칙으로 체크리스트를 유지/재생성.

### 구현 완료
- ✅ 반복 규칙: **매일 / 매주(요일 선택) / 매월(특정 일)**
- ✅ 항목 생성/수정에서 반복 옵션 제공 (RepeatSelector 컴포넌트)
- ✅ 반복 규칙 적용 시 다음 실행일을 자동 계산 (RepeatService)
- ✅ 앱 시작 시 due된 항목 자동 재활성화 (processRepeats)

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `repeat_type` (none | daily | weekly | monthly)
  - `repeat_detail` (JSON: 요일 배열 [0-6] / 일자 배열 [1-31])
  - `next_due_at` (다음 활성화 시점, YYYY-MM-DD)
  - `last_completed_at` (마지막 완료 시점)

### UX 요약
- 반복 없는 항목: 수동 체크 후 유지
- 반복 있는 항목: 완료 시 **다음 주기로 next_due_at 설정 → 해당 날짜에 자동 재활성화**
- 항목에 반복 아이콘 표시
- 메모 드로어에서 반복 설정 확인/수정 가능

## 0.3.0 — 스트릭 히트맵(잔디) ✅ 완료
**목표:** 깃허브 잔디형 시각화로 항목별 스트릭을 보여주기.

### 구현 완료
- ✅ 연 단위 히트맵 캘린더 UI (최근 365일)
- ✅ 완료 여부에 따라 **색 농도 2단계** (미완료/완료)
- ✅ **항목별 추적 히트맵**: 사용자가 지정한 항목만 개별 히트맵 표시
- ✅ 스트릭 통계: 총 완료 일수, 현재 스트릭, 최장 스트릭
- ✅ FloatingActions에 스트릭 버튼 추가 (불꽃 아이콘)

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `track_streak` (스트릭 추적 여부, 0 또는 1)
- `completion_logs` (항목별 날짜 단위 로그 테이블)
  - `item_id` (추적 대상 항목 ID)
  - `completed_on` (YYYY-MM-DD)
  - `completed_count` (해당 날짜 완료 건수)
  - PRIMARY KEY (item_id, completed_on)

### UX 요약
- FloatingActions 메뉴에서 스트릭 버튼 클릭 → 모달 표시
- 추적 중인 항목이 없으면 빈 상태 메시지 표시
- 항목 탭 선택 → 해당 항목의 히트맵과 통계 표시
- MemoDrawer에서 **"스트릭 추적" 토글** 제공

## 0.4.1 — 클라우드 동기화 ✅ 완료
**목표:** 멀티 디바이스 연동의 최소 기능 제공.

### 구현 완료
- ✅ **인증**: Apple Sign In (iOS)
- ✅ **동기화 대상**: 카테고리, 항목, 완료 상태, 반복 규칙, **스트릭 기록(completion_logs)**
- ✅ **충돌 해결**: 최신 수정 시간(`updated_at`) 우선
- ✅ **삭제 동기화**: Soft delete 후 서버 삭제 → 로컬 영구 삭제
- ✅ **수동 동기화**: 설정 화면에서 동기화 버튼 제공
- ✅ **강제 풀**: 로컬 데이터 초기화 후 서버에서 다시 가져오기
- ✅ **로그인 상태 유지**: 앱 시작 시 세션 복원 + 만료 시 자동 refresh
- ✅ **실시간 자동 초기화**: 초기화 시간에 타이머로 즉시 적용

- ✅ **Google Sign In**: OAuth PKCE 로그인 (Desktop + iOS/Android)
  - Desktop: localhost 콜백 서버 (`tauri-plugin-oauth`)
  - Mobile: 딥 링크 콜백 (`tickly://auth/callback`)

### 미구현 (향후 개선)
- ⬜ **Realtime 동기화**: WebSocket 기반 실시간 변경 감지

### 기술 스택
- **Backend**: Supabase (PostgreSQL + REST API)
- **인증**: `tauri-plugin-sign-in-with-apple` (iOS), Google OAuth PKCE
- **딥 링크**: `tauri-plugin-deep-link` (iOS/Android OAuth 콜백)
- **HTTP Client**: `reqwest` (Rust)
- **환경변수**: 빌드 타임 주입 (`build.rs` + `cargo:rustc-env`)

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `sync_id` (Supabase UUID)
  - `created_at`, `updated_at` (ISO 8601)
  - `sync_status` (pending | synced | deleted)
- `categories` 테이블 확장:
  - `sync_id`, `created_at`, `updated_at`, `sync_status`
- 새 테이블:
  - `auth_session` (로컬 인증 세션 저장)
  - `sync_metadata` (마지막 동기화 시간 등)

### Supabase 테이블 구조
```sql
-- categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    display_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- todos
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    done BOOLEAN DEFAULT FALSE,
    display_order INTEGER DEFAULT 0,
    memo TEXT,
    repeat_type TEXT DEFAULT 'none',
    repeat_detail TEXT,
    next_due_at TIMESTAMPTZ,
    last_completed_at TIMESTAMPTZ,
    track_streak BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- completion_logs (스트릭 기록)
CREATE TABLE completion_logs (
    id TEXT PRIMARY KEY,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    completed_on TEXT NOT NULL,
    completed_count INTEGER NOT NULL DEFAULT 1
);

-- RLS 정책
ALTER TABLE categories ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own categories" ON categories FOR ALL USING (auth.uid() = user_id);

ALTER TABLE todos ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own todos" ON todos FOR ALL USING (auth.uid() = user_id);

ALTER TABLE completion_logs ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own completion_logs" ON completion_logs FOR ALL USING (auth.uid() = user_id);
```

### 동기화 흐름
1. **Push**: 로컬 변경사항(pending/deleted) → Supabase
2. **Pull**: Supabase 최신 데이터 → 로컬 (updated_at 비교)
3. **삭제 처리**:
   - 동기화된 항목 삭제 → `sync_status = 'deleted'`
   - 동기화 시 서버에서 삭제 → 로컬 영구 삭제

### UX 요약
- 설정 > 클라우드 동기화 메뉴
- 로그인 상태, 동기화 활성화 토글
- 마지막 동기화 시간, 대기 중인 변경사항 수 표시
- "지금 동기화" 버튼, "서버에서 다시 가져오기" 버튼

## 0.5.0 — 공유 리스트
**목표:** 가족/팀 단위 체크리스트 공유.

### MVP 범위
- 공유 단위: 카테고리
- 초대 방식: 링크 공유 (편집 권한만 제공)
- 공유된 리스트에서 항목 추가/완료 가능

### 데이터 모델 변경
- `categories`
  - `share_id`
  - `owner_id`
- `category_members` (공유 멤버 관리)

### UX 요약
- 카테고리 메뉴에서 “공유” 버튼 제공

## 0.6.0 — 위젯
**목표:** 앱을 열지 않고 빠르게 체크.

### MVP 범위
- iOS 홈 위젯 1종 (Lock Screen 제외)
- 오늘의 체크리스트 표시 및 빠른 완료 토글
- 앱과 위젯 데이터 공유 (App Group)

### 데이터 모델 변경
- App Group용 스토리지/캐시 구조 추가

### UX 요약
- 위젯에서 체크 → 앱 내 상태 즉시 반영
