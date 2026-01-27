# Troubleshooting

이 문서는 Tickly 개발 중 발견된 문제와 해결 방법을 기록합니다.

---

## v0.2.0 - 반복 규칙 (Repeat Rules)

### 반복 항목이 자동으로 재활성화되지 않음

**증상**: 매일/매주/매월 반복 설정한 항목이 다음날 앱을 열어도 자동으로 체크 해제되지 않음

**원인**: `processRepeats()`가 `onMount`에서만 호출되어, iOS에서 앱이 백그라운드에서 foreground로 돌아올 때 재실행되지 않음

**해결**: `visibilitychange` 이벤트 리스너 추가 (`src/routes/+page.svelte`)

```typescript
// 앱이 foreground로 돌아올 때 처리
function handleVisibilityChange() {
  if (document.visibilityState === 'visible') {
    processRepeatsAndReload();
  }
}

onMount(async () => {
  // ... 초기화 코드
  document.addEventListener('visibilitychange', handleVisibilityChange);
});

onDestroy(() => {
  document.removeEventListener('visibilitychange', handleVisibilityChange);
});
```

**테스트 방법**:
```bash
# 1. 테스트용 반복 항목 추가 (done=1, next_due_at=오늘)
sqlite3 "/Users/u-keunsong/Library/Application Support/com.u-keunsong.tickly/tickly.db" \
  "INSERT INTO todos (text, done, category_id, display_order, repeat_type, next_due_at)
   VALUES ('테스트 반복', 1, 1, 9999, 'daily', '$(date +%Y-%m-%d)');"

# 2. 앱 실행
yarn tauri dev

# 3. 해당 항목이 자동으로 체크 해제되는지 확인
```

**커밋**: (수정 완료, 커밋 대기)
