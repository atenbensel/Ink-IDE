import React, { useState } from 'react';
import './App.css';

const App = () => {
  const [activeTab, setActiveTab] = useState('editor');

  const handleTabChange = (tab) => {
    setActiveTab(tab);
  };

  return (
    <div className="App">
      <header className="App-header">
        <nav>
          <button onClick={() => handleTabChange('editor')}>Editor</button>
          <button onClick={() => handleTabChange('deployment')}>Deployment</button>
          <button onClick={() => handleTabChange('debugger')}>Debugger</button>
        </nav>
      </header>

      <main className="App-main">
        {activeTab === 'editor' && <CodeEditor />}
        {activeTab === 'deployment' && <DeploymentManager />}
        {activeTab === 'debugger' && <VisualDebugger />}
      </main>
    </div>
  );
};

export default App;

const CodeEditor = () => {
  return (
    <div className="CodeEditor">
      <MonacoEditor
        height="500px"
        language="ink"
        options={{
          minimap: {
            enabled: true,
          },
        }}
      />
    </div>
  );
};

const DeploymentManager = () => {
  return (
    <div className="DeploymentManager">
      <select>
        <option value="aleph-zero-mainnet">Aleph Zero Mainnet</option>
        <option value="aleph-zero-testnet">Aleph Zero Testnet</option>
      </select>
      <label>Gas Limit:</label>
      <input type="number" />
      <label>Transaction Fee:</label>
      <input type="number" />
      <button>Deploy</button>
    </div>
  );
};

const VisualDebugger = () => {
  return (
    <div className="VisualDebugger">
      <div className="Timeline">
        <div className="Breakpoint"></div>
        <div className="Breakpoint"></div>
        <div className="Breakpoint"></div>
      </div>

      <div className="VariablePanel">
        <div className="Variable">
          <span>variable1:</span>
          <span>value1</span>
        </div>
        <div className="Variable">
          <span>variable2:</span>
          <span>value2</span>
        </div>
      </div>

      <div className="ExecutionControls">
        <button>Step Into</button>
        <button>Step Over</button>
        <button>Step Out</button>
        <button>Resume</button>
      </div>

      <div className="StackTracePanel">
        <div className="StackFrame">
          <span>function1</span>
          <span>contract1.ink:10</span>
        </div>
        <div className="StackFrame">
          <span>function2</span>
          <span>contract2.ink:20</span>
        </div>
      </div>
    </div>
  );
};
