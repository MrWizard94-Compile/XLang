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
  applyStructuralEdit,
  compileSource,
  getOllamaStatus,
  inspectStructure,
  isDesktopRuntime,
  reviewSource,
  type AuthoringDiagnostic,
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

function runtimeSummary(result: CompileResponse | null): string {
  if (!result?.success) {
    return "Build an Aether artifact to inspect VM output.";
  }

  const stdout = result.runtimeOutput || "(no stdout)";
  return stdout + "\n\nExit code: " + String(result.exitCode);
}

function authoringDiagnosticMessage(diagnostic: AuthoringDiagnostic | null): string {
  if (!diagnostic) {
    return "The structural authoring request did not return a diagnostic.";
  }

  return `${diagnostic.code} at line ${diagnostic.span.line}, column ${diagnostic.span.column}: ${diagnostic.message}`;
}

function App() {
  const [source, setSource] = useState(loadSource);
  const [selectedModel, setSelectedModel] = useState(loadModel);
  const [ollama, setOllama] = useState<OllamaStatus>(initialStatus);
  const [compileResult, setCompileResult] = useState<CompileResponse | null>(null);
  const [review, setReview] = useState<ReviewResponse | null>(null);
  const [structure, setStructure] = useState<string | null>(null);
  const [structuralEdit, setStructuralEdit] = useState("");
  const [operationError, setOperationError] = useState<string | null>(null);
  const [compiling, setCompiling] = useState(false);
  const [reviewing, setReviewing] = useState(false);
  const [inspectingStructure, setInspectingStructure] = useState(false);
  const [applyingStructuralEdit, setApplyingStructuralEdit] = useState(false);
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

  async function runStructureInspection() {
    setInspectingStructure(true);
    setOperationError(null);
    try {
      const response = await inspectStructure(source);
      if (response.success && response.document) {
        setStructure(response.document);
      } else {
        setStructure(null);
        setOperationError(authoringDiagnosticMessage(response.diagnostic));
      }
    } catch (error) {
      setStructure(null);
      setOperationError(errorMessage(error));
    } finally {
      setInspectingStructure(false);
    }
  }

  async function runStructuralEdit() {
    setApplyingStructuralEdit(true);
    setOperationError(null);
    try {
      const response = await applyStructuralEdit(source, structuralEdit);
      if (response.success && response.source && response.document) {
        setSource(response.source);
        setStructure(response.document);
        setCompileResult(null);
        setReview(null);
      } else {
        setOperationError(authoringDiagnosticMessage(response.diagnostic));
      }
    } catch (error) {
      setOperationError(errorMessage(error));
    } finally {
      setApplyingStructuralEdit(false);
    }
  }

  function restoreExample() {
    setSource(loadSource());
    setCompileResult(null);
    setReview(null);
    setStructure(null);
    setStructuralEdit("");
    setOperationError(null);
  }

  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="brand" aria-label="Aether Studio">
          <Braces size={24} strokeWidth={2.2} />
          <span>AETHER</span>
          <strong>STUDIO</strong>
        </div>
        <div className="topbar-status" data-online={ollama.available && isDesktopRuntime}>
          {ollama.available && isDesktopRuntime ? <Wifi size={15} /> : <WifiOff size={15} />}
          <span>{localState}</span>
        </div>
      </header>

      <section className="commandbar" aria-label="Aether commands">
        <div className="document-name">
          <FileCode2 size={17} />
          <span>workspace.ae</span>
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
            className="command-button structure-button"
            type="button"
            onClick={() => void runStructureInspection()}
            disabled={!isDesktopRuntime || inspectingStructure}
          >
            {inspectingStructure ? <LoaderCircle className="spin" size={17} /> : <Braces size={17} />}
            <span>Structure</span>
          </button>
          <button
            className="command-button compile-button"
            type="button"
            onClick={() => void runCompile()}
            disabled={!isDesktopRuntime || compiling}
          >
            {compiling ? <LoaderCircle className="spin" size={17} /> : <Play size={17} />}
            <span>Build</span>
          </button>
        </div>
      </section>

      <main className="workspace">
        <aside className="navigator" aria-label="Aether workspace navigator">
          <div className="panel-label">Workspace</div>
          <div className="file-entry" data-active="true">
            <FileCode2 size={17} />
            <span>workspace.ae</span>
          </div>
          <div className="navigator-footer">
            <Terminal size={16} />
            <span>Verified AETH VM</span>
          </div>
        </aside>

        <section className="editor-pane" aria-label="Aether source editor">
          <div className="pane-heading">
            <span>Source</span>
            <button
              className="icon-button"
              type="button"
              onClick={restoreExample}
              title="Restore saved Aether source"
              aria-label="Restore saved Aether source"
            >
              <RotateCcw size={16} />
            </button>
          </div>
          <textarea
            className="source-editor"
            aria-label="Aether source"
            spellCheck="false"
            value={source}
            onChange={(event) => {
              setSource(event.target.value);
              setStructure(null);
            }}
          />
        </section>

        <aside className="inspector" aria-label="Aether compiler output">
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
                  ? "Verified Aether artifact built and run."
                  : "Build the current Aether source."
              )}
            </div>
          </section>

          <section className="output-section artifact-section">
            <div className="pane-heading">
              <span>AETH Artifact</span>
              <Terminal size={17} />
            </div>
            <pre className="output-content artifact-output">
              {compileResult?.artifact ?? "No verified artifact yet."}
            </pre>
          </section>

          <section className="output-section">
            <div className="pane-heading">
              <span>VM Result</span>
              <Play size={17} />
            </div>
            <pre className="output-content runtime-output">{runtimeSummary(compileResult)}</pre>
          </section>

          <section className="output-section">
            <div className="pane-heading">
              <span>Semantic Structure</span>
              <Braces size={17} />
            </div>
            <pre className="output-content ast-output">
              {structure ?? "Inspect the source to produce its local aether.ast/v1 document."}
            </pre>
          </section>

          <section className="output-section structural-edit-section">
            <div className="pane-heading">
              <span>Structural Edit</span>
              <button
                className="command-button structure-apply-button"
                type="button"
                onClick={() => void runStructuralEdit()}
                disabled={!isDesktopRuntime || applyingStructuralEdit || !structuralEdit.trim()}
              >
                {applyingStructuralEdit ? <LoaderCircle className="spin" size={16} /> : <CheckCircle2 size={16} />}
                <span>Apply</span>
              </button>
            </div>
            <textarea
              className="structural-edit-input"
              aria-label="Aether structural edit JSON"
              spellCheck="false"
              value={structuralEdit}
              onChange={(event) => setStructuralEdit(event.target.value)}
              placeholder="Paste an aether.edit/v1 JSON request. It must match the current canonical source."
            />
          </section>

          <section className="output-section review-section">
            <div className="pane-heading">
              <span>Local Review</span>
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
        <span>Aether compilation and local review are separate processes</span>
      </footer>
    </div>
  );
}

export default App;
