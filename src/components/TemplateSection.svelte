<script lang="ts">
  import type { Template } from '../types';
  import { iosFocusFix } from '$lib/iosFocusFix';

  interface Props {
    templates: Template[];
    onAddTemplate: (text: string) => void;
    onEditTemplate: (id: number, text: string) => void;
    onDeleteTemplate: (id: number) => void;
    onUseTemplate: (templateId: number) => void;
  }

  let { templates, onAddTemplate, onEditTemplate, onDeleteTemplate, onUseTemplate }: Props = $props();

  let isExpanded = $state(false);
  let isAddingTemplate = $state(false);
  let newTemplateText = $state('');
  let editingTemplateId = $state<number | null>(null);
  let editingTemplateText = $state('');

  function startAddTemplate() {
    isAddingTemplate = true;
    newTemplateText = '';
  }

  function saveNewTemplate() {
    const trimmed = newTemplateText.trim();
    if (trimmed) {
      onAddTemplate(trimmed);
      isAddingTemplate = false;
      newTemplateText = '';
    }
  }

  function cancelAddTemplate() {
    isAddingTemplate = false;
    newTemplateText = '';
  }

  function startEditTemplate(template: Template) {
    editingTemplateId = template.id;
    editingTemplateText = template.text;
  }

  function saveEditTemplate() {
    if (editingTemplateId !== null && editingTemplateText.trim()) {
      onEditTemplate(editingTemplateId, editingTemplateText.trim());
      editingTemplateId = null;
      editingTemplateText = '';
    }
  }

  function cancelEditTemplate() {
    editingTemplateId = null;
    editingTemplateText = '';
  }

  function handleKeydown(e: KeyboardEvent, action: 'add' | 'edit') {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (action === 'add') {
        saveNewTemplate();
      } else {
        saveEditTemplate();
      }
    } else if (e.key === 'Escape') {
      e.preventDefault();
      if (action === 'add') {
        cancelAddTemplate();
      } else {
        cancelEditTemplate();
      }
    }
  }
</script>

<div class="border-b border-stroke bg-canvas">
  <!-- Expand/Collapse Header -->
  <button
    onclick={() => (isExpanded = !isExpanded)}
    class="w-full px-4 py-3 flex items-center justify-between hover:bg-mist transition-colors"
  >
    <div class="flex items-center gap-2">
      <svg
        class="w-5 h-5 text-ink-muted transition-transform {isExpanded ? 'rotate-90' : ''}"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
      <span class="text-sm font-medium text-ink">템플릿 ({templates.length})</span>
    </div>
    <span class="text-xs text-ink-muted">자주 사용하는 항목을 저장하세요</span>
  </button>

  {#if isExpanded}
    <div class="px-4 pb-3">
      <!-- Template List -->
      <div class="space-y-2">
        {#each templates as template (template.id)}
          <div class="flex items-center gap-2 bg-paper rounded px-3 py-2 border border-stroke">
            {#if editingTemplateId === template.id}
              <!-- Edit Mode -->
              <input
                use:iosFocusFix
                bind:value={editingTemplateText}
                onkeydown={(e) => handleKeydown(e, 'edit')}
                onblur={saveEditTemplate}
                class="flex-1 px-2 py-1 text-sm border border-accent-sky-strong rounded text-ink focus:outline-none focus:ring-1 focus:ring-accent-sky-strong"
                type="text"
                autofocus
              />
            {:else}
              <!-- Display Mode -->
              <button
                onclick={() => onUseTemplate(template.id)}
                class="flex-1 text-left text-sm text-ink hover:text-accent-sky-strong transition-colors"
              >
                {template.text}
              </button>
              <button
                onclick={() => startEditTemplate(template)}
                class="p-1 text-ink-muted hover:text-accent-sky-strong transition-colors"
                title="수정"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                  />
                </svg>
              </button>
              <button
                onclick={() => onDeleteTemplate(template.id)}
                class="p-1 text-ink-muted hover:text-accent-peach-strong transition-colors"
                title="삭제"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            {/if}
          </div>
        {/each}

        <!-- Add Template Input -->
        {#if isAddingTemplate}
          <div class="flex gap-2 bg-paper rounded px-3 py-2 border-2 border-accent-sky-strong">
            <input
              use:iosFocusFix
              bind:value={newTemplateText}
              onkeydown={(e) => handleKeydown(e, 'add')}
              placeholder="템플릿 이름..."
              class="flex-1 px-2 py-1 text-sm text-ink placeholder:text-ink-muted focus:outline-none"
              type="text"
              autofocus
            />
            <button
              onclick={saveNewTemplate}
              class="px-3 py-1 text-sm text-ink bg-accent-sky-strong hover:bg-accent-sky rounded"
            >
              저장
            </button>
            <button
              onclick={cancelAddTemplate}
              class="px-3 py-1 text-sm text-ink-muted bg-mist hover:bg-stroke rounded"
            >
              취소
            </button>
          </div>
        {:else}
          <button
            onclick={startAddTemplate}
            class="w-full px-3 py-2 text-sm text-accent-sky-strong border-2 border-dashed border-accent-sky rounded hover:bg-accent-sky transition-colors"
          >
            + 템플릿 추가
          </button>
        {/if}

        {#if templates.length === 0 && !isAddingTemplate}
          <p class="text-center text-sm text-ink-muted py-4">
            템플릿이 없습니다.<br />
            자주 사용하는 항목을 템플릿으로 저장하세요.
          </p>
        {/if}
      </div>
    </div>
  {/if}
</div>
