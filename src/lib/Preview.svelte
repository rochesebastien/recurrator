<script lang="ts">
  import MarkdownIt from 'markdown-it';
  import mermaid from 'mermaid';

  let { content }: { content: string } = $props();

  const md = new MarkdownIt({
    html: true,
    linkify: true,
    typographer: true,
    breaks: false,
  });

  const defaultFence = md.renderer.rules.fence!;
  md.renderer.rules.fence = (tokens, idx, options, env, slf) => {
    const token = tokens[idx];
    if (token.info.trim() === 'mermaid') {
      const escaped = token.content
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');
      return `<div class="mermaid">${escaped}</div>`;
    }
    return defaultFence(tokens, idx, options, env, slf);
  };

  let mermaidReady = false;

  function ensureMermaid() {
    if (mermaidReady) return;
    mermaid.initialize({
      startOnLoad: false,
      theme: 'dark',
      securityLevel: 'loose',
      fontFamily: 'inherit',
    });
    mermaidReady = true;
  }

  let html = $derived(md.render(content));
  let container: HTMLDivElement;
  let renderToken = 0;

  $effect(() => {
    if (!container) return;
    void html;
    const myToken = ++renderToken;
    const nodes = container.querySelectorAll<HTMLElement>('.mermaid');
    if (nodes.length === 0) return;
    ensureMermaid();
    mermaid
      .run({ nodes: Array.from(nodes), suppressErrors: true })
      .catch(() => {
        if (renderToken !== myToken) return;
      });
  });
</script>

<div class="preview" bind:this={container}>{@html html}</div>

<style>
  .preview {
    height: 100%;
    overflow-y: auto;
    padding: 22px 28px 32px;
    color: var(--text);
    font-family: var(--font-sans);
    font-size: 14.5px;
    line-height: 1.65;
  }
</style>
