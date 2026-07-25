import { useCallback, useEffect, useMemo, useState } from "react";
import {
  Bot,
  Braces,
  CheckCircle2,
  CircleAlert,
  FileCode2,
  LoaderCircle,
  Play,
  RefreshCw,
  RotateCcw,
  Sparkles,
  Terminal,
  Wifi,
  WifiOff
} from "lucide-react";
import { DEFAULT_MODEL, chooseModel } from "./lib/models";
import {
  compileSource,
  getOllamaStatus,
  isDesktopRuntime,
  reviewSource,
  type CompileResponse,
  type OllamaStatus,
  type ReviewResponse
} from "./services/desktopApi";
import { loadModel, loadSource, saveModel, saveSource } from "./services/persistence";

const initialStatus: OllamaStatus = {
  available: false,
  endpoint: "http://127.0.0.1:11434",
  defaultModel: DEFAULT_MODEL,
  models: [],
  error: null
};

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function App() {
  const [source, setSource] = useState(loadSource);
  const [selectedModel, setSelectedModel] = useState(loadModel);
  const [ollama, setOllama] = useState<OllamaStatus>(initialStatus);
  const [compileResult, setCompileResult] = useState<CompileResponse | null>(null);
  const [review, setReview] = useState<ReviewResponse | null>(null);
  const [operationError, setOperationError] = useState<string | null>(null);
  const [compiling, setCompiling] = useState(false);
  const [reviewing, setReviewing] = useState(false);
  const [refreshing, setRefreshing] = useState(false);

  const refreshOllama = useCallback(async () => {
    if (!isDesktopRuntime) {
      return;
    }

    setRefreshing(true);
    try {
      const status = await getOllamaStatus();
      setOllama(status);
      setSelectedModel((previous) =>
        chooseModel(status.models, previous || status.defaultModel, status.defaultModel)
      );
    } catch (error) {
      setOllama((previous) => ({
        ...previous,
        available: false,
        error: errorMessage(error)
      }));
    } finally {
      setRefreshing(false);
    }
  }, []);

  useEffect(() => {
    void refreshOllama();
  }, [refreshOllama]);

  useEffect(() => {
    saveSource(source);
  }, [source]);

  useEffect(() => {
    saveModel(selectedModel);
  }, [selectedModel]);

  const availableModels = useMemo(() => {
    const models = new Set(ollama.models);
    models.add(selectedModel);
    return Array.from(models).sort((left, right) => left.localeCompare(right));
  }, [ollama.models, selectedModel]);

  const sourceLines = useMemo(() => source.split("\n").length, [source]);
  const localState = isDesktopRuntime
    ? ollama.available
      ? "Ollama online: " + ollama.models.length + " model(s)"
      : "Ollama offline"
    : "Desktop runtime required";

  async function runCompile() {
    setCompiling(true);
    setOperationError(null);
    setReview(null);
    try {
      setCompileResult(await compileSource(source));
    } catch (error) {
      setCompileResult(null);
      setOperationError(errorMessage(error));
    } finally {
      setCompiling(false);
    }
  }

  async function runReview() {
    setReviewing(true);
    setOperationError(null);
    try {
      setReview(await reviewSource(source, selectedModel));
    } catch (error) {
      setReview(null);
      setOperationError(errorMessage(error));
    } finally {
      setReviewing(false);
    }
  }

  function restoreExample() {
    setSource(loadSource());
    setCompileResult(null);
    setReview(null);
    setOperationError(null);
  }

  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="brand" aria-label="XLang Studio">
          <Braces size={24} strokeWidth={2.2} />
          <span>XLANG</span>
          <strong>STUDIO</strong>
        </div>
        <div className="topbar-status" data-online={ollama.available && isDesktopRuntime}>
          {ollama.available && isDesktopRuntime ? <Wifi size={15} /> : <WifiOff size={15} />}
          <span>{localState}</span>
        </div>
      </header>

      <section className="commandbar" aria-label="Compiler commands">
        <div className="document-name">
          <FileCode2 size={17} />
          <span>workspace.xl</span>
          <small>{sourceLines} lines</small>
        </div>
        <div className="commandbar-actions">
          <label className="model-field">
            <Bot size={16} aria-hidden="true" />
            <select
              aria-label="Local Ollama model"
              value={selectedModel}
              onChange={(event) => setSelectedModel(event.target.value)}
              disabled={!isDesktopRuntime || !ollama.available}
            >
              {availableModels.map((model) => (
                <option key={model} value={model}>
                  {model}
                </option>
              ))}
            </select>
          </label>
          <button
            className="icon-button"
            type="button"
            onClick={() => void refreshOllama()}
            disabled={!isDesktopRuntime || refreshing}
            title="Refresh local Ollama models"
            aria-label="Refresh local Ollama models"
          >
            <RefreshCw className={refreshing ? "spin" : undefined} size={17} />
          </button>
          <button
            className="command-button review-button"
            type="button"
            onClick={() => void runReview()}
            disabled={!isDesktopRuntime || !ollama.available || reviewing}
          >
            {reviewing ? <LoaderCircle className="spin" size={17} /> : <Sparkles size={17} />}
            <span>Review</span>
          </button>
          <button
            className="command-button compile-button"
            type="button"
            onClick={() => void runCompile()}
            disabled={!isDesktopRuntime || compiling}
          >
            {compiling ? <LoaderCircle className="spin" size={17} /> : <Play size={17} />}
            <span>Compile</span>
          </button>
        </div>
      </section>

      <main className="workspace">
        <aside className="navigator" aria-label="Workspace navigator">
          <div className="panel-label">Workspace</div>
          <div className="file-entry" data-active="true">
            <FileCode2 size={17} />
            <span>workspace.xl</span>
          </div>
          <div className="navigator-footer">
            <Terminal size={16} />
            <span>Bootstrap frontend</span>
          </div>
        </aside>

        <section className="editor-pane" aria-label="Source editor">
          <div className="pane-heading">
            <span>Source</span>
            <button
              className="icon-button"
              type="button"
              onClick={restoreExample}
              title="Restore saved source"
              aria-label="Restore saved source"
            >
              <RotateCcw size={16} />
            </button>
          </div>
          <textarea
            className="source-editor"
            aria-label="XLang source"
            spellCheck="false"
            value={source}
            onChange={(event) => setSource(event.target.value)}
          />
        </section>

        <aside className="inspector" aria-label="Compiler output">
          <section className="output-section">
            <div className="pane-heading">
              <span>Diagnostics</span>
              {compileResult?.success ? (
                <CheckCircle2 className="success-icon" size={18} />
              ) : (
                <CircleAlert className="warning-icon" size={18} />
              )}
            </div>
            <div className="output-content diagnostic-output">
              {operationError ?? compileResult?.diagnostic ?? (
                compileResult?.success
                  ? "Parsed and type-checked successfully."
                  : "Run compilation to inspect the current source."
              )}
            </div>
          </section>

          <section className="output-section ast-section">
            <div className="pane-heading">
              <span>AST</span>
              <Terminal size={17} />
            </div>
            <pre className="output-content ast-output">
              {compileResult?.ast ?? "No successful compilation yet."}
            </pre>
          </section>

          <section className="output-section review-section">
            <div className="pane-heading">
              <span>Local review</span>
              <Bot size={17} />
            </div>
            <div className="output-content review-output">
              {review ? (
                <>
                  <strong>{review.model}</strong>
                  <p>{review.content}</p>
                </>
              ) : (
                "No local review requested."
              )}
            </div>
          </section>
        </aside>
      </main>

      <footer className="statusbar">
        <span>{ollama.endpoint}</span>
        <span>Compiler and review are separate local processes</span>
      </footer>
    </div>
  );
}

export default App;
