# Tickly

<p align="center">
  <a href="https://apps.apple.com/kr/app/tickly/id6757784860">
    <img src="https://img.shields.io/badge/App_Store-0D96F6?style=for-the-badge&logo=app-store&logoColor=white" alt="App Store" />
  </a>
</p>

## 프로젝트 개요

<p align="center">
<img width="30%" height="30%" alt="스크린샷, 2026-01-20 오후 6 36 09" src="https://github.com/user-attachments/assets/936ab4b0-8ddd-467b-af4a-755d336d59ce" />
</p>

**Tickly**는 심플한 체크리스트 앱입니다. 복잡한 기능 없이 빠르고 간단하게 사용할 수 있도록 설계되었습니다.
현재 릴리즈 버전은 **v0.8.3**입니다.

### 주요 기능

#### 기본 기능
- ✅ 항목 추가/수정/삭제/완료 표시
- 📝 항목별 메모 기능
- 💾 SQLite 영구 저장 (앱 재시작 후에도 데이터 유지)

#### 고급 기능
- 📁 **카테고리 관리** - 상황별 리스트 분리 (집, 여행, 운동 등)
- 👆 **스와이프 삭제** - iOS 네이티브 스타일 제스처
- 🔀 **드래그 정렬** - 항목/카테고리 순서를 자유롭게 변경
- 📊 **자동 정렬** - 완료된 항목이 자동으로 아래로 이동
- 🔄 **반복 규칙** - 매일/매주/매월 자동 재활성화 스케줄링
- 🔥 **스트릭 히트맵** - 항목별 GitHub 스타일 달성 기록 시각화
- 🏷️ **태그** - 항목에 #태그 부착, 태그 기반 필터링, 클라우드 동기화
- 🕸️ **그래프 뷰** - 카테고리·태그·항목 간 관계를 노드 그래프로 시각화 (PixiJS WebGL + d3-force)
- 🔔 **항목 알림** - 항목별 알림 시간 설정, iOS 로컬 알림으로 리마인더
- 🔗 **앱 연결** - 항목에 앱 연결(지도/배달/택시 등) 설정 후 바로 실행
- ☁️ **클라우드 동기화** - Apple/Google 로그인으로 멀티 디바이스 실시간 연동
- 🎨 **테마 커스터마이징** - 5가지 프리셋 + 커스텀 색상 지원
- 🔤 **폰트 커스터마이징** - 다양한 폰트 선택 가능
- 🌐 **다국어 지원** - 한국어/영어/일본어 지원

#### 플랫폼
- 📱 iOS 네이티브 앱 지원
- 🖥️ macOS, Windows, Linux 데스크톱 지원
- 🎨 심플하고 깔끔한 UI
- 🚀 빠른 성능

## 기술 스택

- **Frontend**: SvelteKit (Svelte 5 + TypeScript)
- **Backend**: Rust (Tauri v2)
- **Database**: SQLite (rusqlite) + Supabase (PostgreSQL)
- **Cloud**: Supabase (인증, 데이터 동기화)
- **Styling**: TailwindCSS
- **Platform**: iOS, macOS, Windows, Linux

## 아키텍처

### 계층형 아키텍처

프로젝트는 유지보수성과 관심사 분리를 위해 계층형 아키텍처를 따릅니다:

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend                              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   Routes    │ -> │ Components  │ -> │  API Layer  │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │ invoke()
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Rust Backend                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │  Commands   │ -> │  Services   │ -> │ Repository  │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                               │              │
│                                               ▼              │
│                                        ┌─────────────┐      │
│                                        │   SQLite    │      │
│                                        └─────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## 개발 환경 설정

### 필수 요구사항

- Node.js 18+
- Yarn
- Rust
- Xcode (iOS 개발용, macOS만)

### 설치

```bash
# 의존성 설치
yarn install

# 프론트엔드 빌드
yarn build
```

## 실행 방법

### 데스크톱 (개발 모드)

```bash
yarn tauri dev
```

### iOS

#### 초기 설정 (한 번만)

```bash
# iOS 프로젝트 초기화
yarn tauri ios init

# 위젯 타깃/설정 동기화
yarn ios:widget:setup

# Xcode에서 개발팀 설정
open src-tauri/gen/apple/tickly.xcodeproj
# Signing & Capabilities > Team 선택
```

위젯 App Group 기본값은 `group.com.u-keunsong.tickly` 입니다. 변경하려면 `/Users/u-keunsong/Desktop/Projects/Tickly/src-tauri/Info.ios.plist`, `/Users/u-keunsong/Desktop/Projects/Tickly/src-tauri/ios-widget/tickly_iOS.entitlements`, `/Users/u-keunsong/Desktop/Projects/Tickly/src-tauri/ios-widget/TicklyWidgetExtension/Info.plist`, `/Users/u-keunsong/Desktop/Projects/Tickly/src-tauri/ios-widget/TicklyWidgetExtension/TicklyWidgetExtension.entitlements`를 동일한 값으로 맞춰주세요.

iOS 17 이상에서는 위젯에서 카테고리 체크 버튼을 눌러 앱을 열지 않고 항목을 완료 처리할 수 있습니다. 위젯에서 누른 액션은 App Group 큐 파일에 저장되고, 앱 실행 시 자동 반영됩니다.

#### 카테고리 위젯 동작

- 지원 사이즈: Small / Medium / Large
- Small: 카테고리 이름 + 리프레시 버튼 + 체크 가능한 항목 + `완료/전체` + `+N more`
- Medium / Large: 카테고리 이름 + 리프레시 버튼 + 체크 가능한 항목 + `% done` + `완료/전체`
- 항목 태그: 위젯 항목에 태그를 표시 (`#태그`, 다수일 때 `#첫태그 +N`)
- 테마 연동: 앱 테마(프리셋/커스텀 색상)가 위젯 배경/텍스트 색상에 동기화

#### 위젯 개발 시 주의

- `src-tauri/ios-widget/` 아래 템플릿을 수정한 뒤에는 반드시 `yarn ios:widget:setup`을 실행해야 `src-tauri/gen/apple/` 생성본에 반영됩니다.

#### iOS 앱 빌드 및 설치

```bash
# 프론트엔드 빌드
yarn build

# iOS 프로덕션 빌드 (실제 기기 또는 시뮬레이터)
yarn tauri ios build --open
```

Xcode가 자동으로 열리고 빌드가 진행됩니다. 빌드 완료 후 기기에 자동 설치됩니다.

#### 아이폰에서 앱 신뢰 설정

1. 설정 > 일반 > VPN 및 기기 관리
2. 개발자 앱 섹션에서 본인의 Apple ID 선택
3. "신뢰" 버튼 클릭

## 프로젝트 구조

```
Tickly/
├── src/                              # Frontend (SvelteKit)
│   ├── routes/
│   │   ├── +page.svelte              # 메인 페이지
│   │   ├── graph/
│   │   │   └── +page.svelte         # 그래프 뷰 페이지
│   │   └── settings/
│   │       ├── +page.svelte          # 설정 메인 페이지
│   │       ├── theme/
│   │       │   └── +page.svelte      # 테마 설정 페이지
│   │       ├── language/
│   │       │   └── +page.svelte      # 언어 설정 페이지
│   │       └── tags/
│   │           └── +page.svelte      # 태그 관리 페이지
│   ├── components/                   # 재사용 가능한 컴포넌트
│   │   ├── ModalWrapper.svelte       # 공통 모달 레이아웃
│   │   ├── SettingsLayout.svelte     # 공통 설정 페이지 레이아웃
│   │   ├── FloatingActions.svelte    # FAB 버튼 (추가, 그래프, 메뉴)
│   │   ├── CategoryMenuModal.svelte  # 카테고리 메뉴 모달
│   │   ├── GraphCanvas.svelte       # 그래프 뷰 (PixiJS + d3-force)
│   │   ├── LeafTodoItem.svelte       # Todo 항목 컴포넌트
│   │   ├── AddItemModal.svelte       # 항목 추가 모달
│   │   ├── EditItemModal.svelte      # 항목 수정 모달
│   │   ├── LinkedAppModal.svelte     # 앱 연결 선택 모달
│   │   ├── SwipeableItem.svelte      # 스와이프 삭제 래퍼
│   │   ├── CategoryTabs.svelte       # 카테고리 탭
│   │   ├── StreakModal.svelte        # 스트릭 히트맵 모달
│   │   ├── StreakHeatmap.svelte      # 히트맵 그리드 컴포넌트
│   │   ├── TagChip.svelte           # #태그 텍스트 표시
│   │   ├── TagInput.svelte          # 태그 입력 + 자동완성
│   │   ├── TagFilterModal.svelte    # 태그 필터 선택 모달
│   │   └── ...
│   ├── lib/
│   │   ├── api/                      # API 레이어 (Tauri invoke 래퍼)
│   │   │   ├── categoryApi.ts        # Category API
│   │   │   ├── todoApi.ts            # Todo API
│   │   │   ├── settingsApi.ts        # Settings API
│   │   │   ├── streakApi.ts          # Streak API
│   │   │   ├── authApi.ts            # Auth API (로그인/로그아웃)
│   │   │   ├── syncApi.ts            # Sync API (동기화)
│   │   │   ├── realtimeApi.ts        # Realtime API (실시간 연결)
│   │   │   ├── tagApi.ts             # Tag API (태그 CRUD)
│   │   │   └── graphApi.ts           # Graph API (그래프 데이터)
│   │   ├── stores/                   # Svelte 5 reactive stores
│   │   │   ├── appStore.svelte.ts    # 앱 상태 (카테고리, 항목)
│   │   │   ├── modalStore.svelte.ts  # 모달 상태 관리
│   │   │   ├── authStore.svelte.ts   # 인증 상태 관리
│   │   │   └── syncStore.svelte.ts   # 동기화 상태 관리
│   │   ├── i18n/                     # 다국어 지원
│   │   │   ├── i18nStore.svelte.ts   # i18n 스토어
│   │   │   ├── ko.ts                 # 한국어 번역
│   │   │   ├── en.ts                 # 영어 번역
│   │   │   └── ja.ts                 # 일본어 번역
│   │   ├── linkedApps.ts             # 앱 연결 메타데이터/실행 유틸리티
│   │   ├── themes.ts                 # 테마 프리셋 및 유틸리티
│   │   └── notification.ts           # 알림 스케줄링 유틸리티
│   ├── types.ts                      # TypeScript 타입 정의
│   └── app.css                       # TailwindCSS + CSS 변수
├── src-tauri/                        # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── lib.rs                    # 앱 진입점 및 모듈 등록
│   │   ├── models/                   # 데이터 모델
│   │   │   ├── category.rs           # Category 구조체
│   │   │   ├── todo_item.rs          # TodoItem 구조체
│   │   │   ├── completion_log.rs     # CompletionLog 구조체
│   │   │   ├── sync.rs               # Sync 관련 구조체 (AuthSession, SyncResult 등)
│   │   │   ├── tag.rs                # Tag, TodoTag 구조체
│   │   │   └── graph.rs              # GraphNode, GraphEdge, GraphData 구조체
│   │   ├── repository/               # 데이터 접근 레이어
│   │   │   ├── database.rs           # DB 초기화
│   │   │   ├── migration.rs          # 스키마 마이그레이션
│   │   │   ├── category_repo.rs      # Category CRUD
│   │   │   ├── todo_repo.rs          # Todo CRUD
│   │   │   ├── settings_repo.rs      # Settings CRUD
│   │   │   ├── completion_log_repo.rs # CompletionLog CRUD
│   │   │   ├── auth_repo.rs          # Auth 세션 CRUD
│   │   │   ├── sync_repo.rs          # Sync 메타데이터 CRUD
│   │   │   ├── tag_repo.rs           # Tag CRUD
│   │   │   ├── todo_tag_repo.rs      # TodoTag (조인) CRUD
│   │   │   └── graph_repo.rs         # Graph 데이터 집계 쿼리
│   │   ├── service/                  # 비즈니스 로직 레이어
│   │   │   ├── category_service.rs   # Category 비즈니스 로직
│   │   │   ├── todo_service.rs       # Todo 비즈니스 로직
│   │   │   ├── reset_service.rs      # 리셋 로직
│   │   │   ├── repeat_service.rs     # 반복 규칙 로직
│   │   │   ├── streak_service.rs     # 스트릭 추적 로직
│   │   │   ├── auth_service.rs       # 인증 서비스
│   │   │   ├── sync_service.rs       # 동기화 서비스
│   │   │   ├── realtime_service.rs   # 실시간 동기화 (WebSocket)
│   │   │   ├── supabase_client.rs    # Supabase REST API 클라이언트
│   │   │   └── tag_service.rs        # 태그 비즈니스 로직
│   │   └── commands/                 # Tauri 커맨드 핸들러
│   │       ├── category_commands.rs  # Category 커맨드
│   │       ├── todo_commands.rs      # Todo 커맨드
│   │       ├── settings_commands.rs  # Settings 커맨드
│   │       ├── streak_commands.rs    # Streak 커맨드
│   │       ├── auth_commands.rs      # Auth 커맨드
│   │       ├── sync_commands.rs      # Sync 커맨드
│   │       ├── realtime_commands.rs  # Realtime 커맨드
│   │       ├── tag_commands.rs       # Tag 커맨드
│   │       └── graph_commands.rs     # Graph 커맨드
│   └── tauri.conf.json               # Tauri 설정
├── docs/                             # 프로젝트 문서
│   ├── deployment-automation.md      # CI/CD 및 배포 자동화 가이드
│   ├── roadmap.md                    # 버전별 로드맵
│   └── troubleshooting.md            # 트러블슈팅 기록
├── CLAUDE.md                         # 프로젝트 가이드
└── README.md                         # 이 파일
```

## 데이터 저장

### 데스크톱
- **macOS**: `~/Library/Application Support/com.u-keunsong.tickly/tickly.db`
- **Windows**: `C:\Users\[USER]\AppData\Roaming\com.u-keunsong.tickly\tickly.db`
- **Linux**: `~/.config/com.u-keunsong.tickly/tickly.db`

### iOS
앱 샌드박스 내부에 저장 (사용자가 직접 접근 불가)

## 개발 가이드

### Svelte 5 Runes 문법

이 프로젝트는 Svelte 5의 새로운 runes 문법을 사용합니다:

```svelte
<script lang="ts">
  // State
  let count = $state(0);
  let doubled = $derived(count * 2);

  // Props
  let { title, items = [] } = $props();

  // Event handlers (new syntax)
  function handleClick() {
    count++;
  }
</script>

<button onclick={handleClick}>Click</button>
```

### API 레이어 사용

프론트엔드에서 백엔드와 통신할 때는 반드시 API 레이어를 사용합니다:

```typescript
// ✅ 올바른 방법 - API 모듈 사용
import * as todoApi from '$lib/api/todoApi';
import * as categoryApi from '$lib/api/categoryApi';

const items = await todoApi.getItems(categoryId);
const newItem = await todoApi.addItem('우유 사기', categoryId);
await todoApi.toggleItem(itemId);
```

```typescript
// ❌ 잘못된 방법 - 직접 invoke 호출
import { invoke } from '@tauri-apps/api/core';
const items = await invoke('get_items', { categoryId });
```

### Rust 백엔드 레이어

1. **Models**: 데이터 구조체 정의
2. **Repository**: 데이터베이스 CRUD 작업
3. **Service**: 비즈니스 로직 (여러 Repository 조합)
4. **Commands**: Tauri 커맨드 (Service 호출)

## 빌드

### 데스크톱

```bash
yarn tauri build
```

### iOS (프로덕션)

```bash
yarn build
yarn tauri ios build --open
```

생성된 `.ipa` 파일은 `src-tauri/gen/apple/build/` 디렉토리에 저장됩니다.

## 배포

### iOS App Store

**Tickly**는 iOS App Store에 배포되어 있습니다.

[![App Store에서 다운로드](https://img.shields.io/badge/App_Store-다운로드-0D96F6?style=flat&logo=app-store&logoColor=white)](https://apps.apple.com/kr/app/tickly/id6757784860)

#### 직접 빌드하려면

1. Apple Developer Program 가입 ($99/년)
2. App Store Connect에서 앱 등록
3. 프로덕션 빌드 생성
4. Xcode에서 Archive 및 업로드
5. App Store Connect에서 메타데이터 작성
6. 앱 심사 제출

자세한 내용은 `CLAUDE.md` 파일을 참고하세요.

## 주의사항

### 무료 Apple ID로 개발

- 앱이 7일 후 만료됩니다
- 재설치 필요: `yarn tauri ios build --open`
- 유료 개발자 계정이 있으면 1년 유효

### iOS 개발 모드

- `yarn tauri ios dev`는 개발 서버가 필요하므로 네트워크 설정 복잡
- **권장**: `yarn tauri ios build --open`로 프로덕션 빌드 사용

## 라이선스

MIT

## 작성자

u-keunsong

---

Made with ❤️ using Tauri + SvelteKit
