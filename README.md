# Tickly

외출할 때 챙겨야 할 물건을 잊지 않도록 도와주는 심플한 체크리스트 앱

## 프로젝트 개요

**Tickly**는 외출 전 체크리스트를 관리하는 미니멀한 Todo 앱입니다. 복잡한 기능 없이 빠르고 간단하게 사용할 수 있도록 설계되었습니다.

### 주요 기능

#### 기본 기능
- ✅ 항목 추가/수정/삭제/완료 표시
- 📝 항목별 메모 기능
- 💾 SQLite 영구 저장 (앱 재시작 후에도 데이터 유지)
- 🔄 자동 일일 초기화 (매일 체크가 자동으로 리셋)

#### 고급 기능
- 📁 **카테고리 관리** - 상황별 리스트 분리 (집, 여행, 운동 등)
- 👆 **스와이프 삭제** - iOS 네이티브 스타일 제스처
- 🔀 **드래그 정렬** - 항목/카테고리 순서를 자유롭게 변경
- 📊 **자동 정렬** - 완료된 항목이 자동으로 아래로 이동
- 🎨 **테마 커스터마이징** - 5가지 프리셋 + 커스텀 색상 지원

#### 플랫폼
- 📱 iOS 네이티브 앱 지원
- 🖥️ macOS, Windows, Linux 데스크톱 지원
- 🎨 심플하고 깔끔한 UI
- 🚀 빠른 성능

## 기술 스택

- **Frontend**: SvelteKit (Svelte 5 + TypeScript)
- **Backend**: Rust (Tauri v2)
- **Database**: SQLite (rusqlite)
- **Styling**: TailwindCSS
- **Platform**: iOS, macOS, Windows, Linux

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

# Xcode에서 개발팀 설정
open src-tauri/gen/apple/tickly.xcodeproj
# Signing & Capabilities > Team 선택
```

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
├── src/                          # 프론트엔드 소스
│   ├── routes/
│   │   ├── +page.svelte          # 메인 페이지
│   │   └── settings/
│   │       ├── +page.svelte      # 설정 메인 페이지
│   │       └── theme/
│   │           └── +page.svelte  # 테마 설정 페이지
│   ├── components/
│   │   ├── LeafTodoItem.svelte   # Todo 항목 컴포넌트
│   │   ├── AddItemModal.svelte   # 항목 추가 모달
│   │   ├── SwipeableItem.svelte  # 스와이프 삭제 래퍼
│   │   ├── CategoryTabs.svelte   # 카테고리 탭
│   │   ├── ColorPicker.svelte    # 색상 선택기
│   │   └── ThemePreview.svelte   # 테마 미리보기
│   ├── lib/
│   │   └── themes.ts             # 테마 프리셋 및 유틸리티
│   ├── types.ts                  # TypeScript 타입 정의
│   └── app.css                   # TailwindCSS + CSS 변수
├── src-tauri/                    # Rust 백엔드
│   ├── src/
│   │   └── lib.rs                # Todo CRUD 로직 + SQLite
│   ├── tauri.conf.json           # Tauri 설정
│   ├── Cargo.toml                # Rust 의존성
│   └── gen/apple/                # iOS 프로젝트 파일
├── CLAUDE.md                     # 프로젝트 가이드
└── README.md                     # 이 파일
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

### Tauri Commands

백엔드 함수는 `#[tauri::command]` 속성으로 정의되고, 프론트엔드에서 `invoke()`로 호출합니다:

```rust
// Rust
#[tauri::command]
fn add_item(text: String, state: State<AppState>) -> Result<TodoItem, String> {
    // ...
}
```

```typescript
// TypeScript
import { invoke } from '@tauri-apps/api/core';

const newItem = await invoke<TodoItem>('add_item', { text: 'Buy milk' });
```

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
