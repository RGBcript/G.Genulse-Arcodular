import { useState, useEffect, useRef } from 'react'
import { FileText, Image, Music, MousePointerClick, Keyboard, Code, FileUp, FileDown } from 'lucide-react'
import init, {
  togen_from_string,
  togen_from_action,
  togen_from_code,
  togen_from_bytes_image,
  togen_from_bytes_audio,
  ToGen
} from '../pkg/genesis_togenizer.js'

// Inicializar WASM
await init()

type TogenResult = {
  hex: string
  header: number
  meta: number
  semantico: number
  estructural: number
  exacto: bigint
}

export default function App() {
  const [inputText, setInputText] = useState('')
  const [togenResult, setTogenResult] = useState<TogenResult | null>(null)
  const [isDragging, setIsDragging] = useState(false)
  const [fileName, setFileName] = useState('')
  const [fileType, setFileType] = useState('')
  const [activeTab, setActiveTab] = useState<'text' | 'file'>('text')
  const fileInputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    // Inicializar WASM al cargar la página
    init().then(() => {
      console.log('WASM initialized')
    })
  }, [])

  const processText = () => {
    if (!inputText.trim()) return

    // Detectar tipo de contenido
    if (inputText.startsWith('ACT:')) {
      const command = inputText.substring(4)
      const hex = togen_from_action(command)
      decodeTogen(hex)
    } else if (inputText.startsWith('CODE:')) {
      const code = inputText.substring(5)
      const hex = togen_from_code(code)
      decodeTogen(hex)
    } else {
      const hex = togen_from_string(inputText)
      decodeTogen(hex)
    }
  }

    const decodeTogen = (hex: string) => {
    // The TOGEN is a 128-bit value in big-endian format
    // We need to parse it correctly from the hex string
    // The hex string is 32 characters (128 bits)
    
    // Parse as bigint first, then extract components
    const fullValue = BigInt('0x' + hex);
    
    // Extract components using bit shifting
    const header = Number(fullValue & 0xFFn);
    const meta = Number((fullValue >> 8n) & 0xFFn);
    const semantico = Number((fullValue >> 16n) & 0xFFFFFFFFn);
    const estructural = Number((fullValue >> 48n) & 0xFFFFFFFFn);
    const exacto = (fullValue >> 80n) & 0xFFFFFFFFFFFFn;
    
    setTogenResult({
      hex,
      header,
      meta,
      semantico,
      estructural,
      exacto
    })
  }

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return
    processFile(file)
  }

  const processFile = (file: File) => {
    setFileName(file.name)
    setFileType(getFileType(file.name))
    
    const reader = new FileReader()
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer
      const uint8Array = new Uint8Array(arrayBuffer)
      
      const fileExtension = file.name.split('.').pop()?.toLowerCase()
      let hex = ''
      
      if (['jpg', 'jpeg', 'png', 'gif', 'webp'].includes(fileExtension || '')) {
        hex = togen_from_bytes_image(uint8Array)
      } else if (fileExtension === 'wav') {
        hex = togen_from_bytes_audio(uint8Array)
      } else {
        // Leer como texto
        const textReader = new FileReader()
        textReader.onload = () => {
          const text = textReader.result as string
          const textHex = togen_from_string(text)
          decodeTogen(textHex)
        }
        textReader.readAsText(file)
        return
      }
      
      decodeTogen(hex)
    }
    
    if (file.type.startsWith('image/')) {
      reader.readAsArrayBuffer(file)
    } else if (file.type.startsWith('audio/')) {
      reader.readAsArrayBuffer(file)
    } else {
      reader.readAsArrayBuffer(file)
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(true)
  }

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
    
    const file = e.dataTransfer.files?.[0]
    if (file) {
      processFile(file)
    }
  }

  const getFileType = (filename: string): string => {
    const extension = filename.split('.').pop()?.toLowerCase()
    
    if (['jpg', 'jpeg', 'png', 'gif', 'webp'].includes(extension || '')) {
      return 'image'
    } else if (extension === 'wav') {
      return 'audio'
    } else if (['rs', 'py', 'js', 'ts', 'json', 'cpp', 'java', 'go'].includes(extension || '')) {
      return 'code'
    } else {
      return 'text'
    }
  }

  const getFileIcon = () => {
    switch (fileType) {
      case 'image': return <Image className="w-8 h-8 text-cyan-400" />
      case 'audio': return <Music className="w-8 h-8 text-purple-400" />
      case 'code': return <Code className="w-8 h-8 text-green-400" />
      default: return <FileText className="w-8 h-8 text-blue-400" />
    }
  }

  const getHeaderType = (header: number) => {
    switch (header) {
      case 0x01: return { name: 'Text', icon: <FileText className="w-4 h-4" /> }
      case 0x02: return { name: 'Code', icon: <Code className="w-4 h-4" /> }
      case 0x03: return { name: 'Image', icon: <Image className="w-4 h-4" /> }
      case 0x04: return { name: 'Audio', icon: <Music className="w-4 h-4" /> }
      case 0x05: return { name: 'Action', icon: <MousePointerClick className="w-4 h-4" /> }
      default: return { name: 'Unknown', icon: null }
    }
  }

  const getMetaType = (header: number, meta: number) => {
    if (header === 0x05) {
      // Action meta types
      switch (meta) {
        case 0x01: return 'Keyboard'
        case 0x02: return 'Mouse'
        default: return 'Other'
      }
    }
    return 'N/A'
  }

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 p-4 md:p-8">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl md:text-5xl font-bold mb-2">
            <span className="text-cyan-400">GENESIS</span> <span className="text-white">TOGENIZER</span>
          </h1>
          <p className="text-gray-400 text-lg">v1.1 - Multimodal Hashing Protocol</p>
        </div>

        {/* Tabs */}
        <div className="flex mb-6 border-b border-gray-700">
          <button
            className={`px-4 py-2 flex items-center gap-2 ${activeTab === 'text' ? 'border-b-2 border-cyan-400 text-cyan-400' : 'text-gray-400'}`}
            onClick={() => setActiveTab('text')}
          >
            <FileText className="w-4 h-4" />
            Text/Code
          </button>
          <button
            className={`px-4 py-2 flex items-center gap-2 ${activeTab === 'file' ? 'border-b-2 border-cyan-400 text-cyan-400' : 'text-gray-400'}`}
            onClick={() => setActiveTab('file')}
          >
            <FileUp className="w-4 h-4" />
            File Upload
          </button>
        </div>

        {/* Input Area */}
        {activeTab === 'text' ? (
          <div className="mb-6">
            <textarea
              className="w-full h-32 p-4 bg-gray-800 border border-gray-700 rounded-lg text-white font-mono resize-none focus:outline-none focus:border-cyan-400"
              placeholder="Enter text, code, or action (ACT:ClickLeft, CODE:fn main()...)"
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
            />
            <button
              className="mt-4 px-6 py-2 bg-cyan-600 hover:bg-cyan-700 text-white rounded-lg transition-colors flex items-center gap-2"
              onClick={processText}
            >
              <MousePointerClick className="w-4 h-4" />
              Generate Togen
            </button>
          </div>
        ) : (
          <div
            className={`mb-6 border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-all ${isDragging ? 'border-cyan-400 bg-gray-800' : 'border-gray-700 bg-gray-800 hover:border-gray-600'}`}
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            onClick={() => fileInputRef.current?.click()}
          >
            <input
              type="file"
              ref={fileInputRef}
              className="hidden"
              onChange={handleFileChange}
              accept=".txt,.rs,.py,.js,.ts,.json,.cpp,.java,.go,.jpg,.jpeg,.png,.gif,.webp,.wav"
            />
            <div className="mb-4">
              {fileName ? (
                <div className="flex items-center justify-center gap-4">
                  {getFileIcon()}
                  <span className="text-white font-medium">{fileName}</span>
                </div>
              ) : (
                <FileUp className="w-12 h-12 text-gray-400 mx-auto mb-4" />
              )}
            </div>
            <p className="text-gray-400 mb-2">
              {fileName ? 'Click to replace or drop new file' : 'Drag & drop files here or click to browse'}
            </p>
            <p className="text-gray-500 text-sm">
              Supported: Images, Audio, Code, Text
            </p>
          </div>
        )}

        {/* Result */}
        {togenResult && (
          <div className="mt-8">
            <h2 className="text-2xl font-bold mb-4 text-white">Togen Result</h2>
            
            {/* Hex Display */}
            <div className="bg-gray-800 border border-gray-700 rounded-lg p-4 mb-6">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-gray-400">0x</span>
                <span className="text-white font-mono text-lg break-all">
                  {togenResult.hex}
                </span>
                <button
                  className="ml-auto text-gray-400 hover:text-white"
                  onClick={() => navigator.clipboard.writeText(togenResult.hex)}
                >
                  Copy
                </button>
              </div>
            </div>

            {/* Breakdown */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
              {/* Header */}
              <div className="bg-gray-800 border border-gray-700 rounded-lg p-4">
                <div className="flex items-center gap-2 mb-2">
                  {getHeaderType(togenResult.header).icon}
                  <h3 className="text-cyan-400 font-semibold">Header</h3>
                </div>
                <p className="text-white font-mono">0x{togenResult.header.toString(16).toUpperCase().padStart(2, '0')}</p>
                <p className="text-gray-400 text-sm mt-1">
                  {getHeaderType(togenResult.header).name}
                </p>
              </div>

              {/* Meta */}
              <div className="bg-gray-800 border border-gray-700 rounded-lg p-4">
                <h3 className="text-purple-400 font-semibold mb-2">Meta</h3>
                <p className="text-white font-mono">0x{togenResult.meta.toString(16).toUpperCase().padStart(2, '0')}</p>
                <p className="text-gray-400 text-sm mt-1">
                  {getMetaType(togenResult.header, togenResult.meta)}
                </p>
              </div>

              {/* Semántico */}
              <div className="bg-gray-800 border border-gray-700 rounded-lg p-4">
                <h3 className="text-green-400 font-semibold mb-2">Semántico</h3>
                <p className="text-white font-mono">0x{togenResult.semantico.toString(16).toUpperCase().padStart(8, '0')}</p>
              </div>

              {/* Estructural */}
              <div className="bg-gray-800 border border-gray-700 rounded-lg p-4">
                <h3 className="text-blue-400 font-semibold mb-2">Estructural</h3>
                <p className="text-white font-mono">0x{togenResult.estructural.toString(16).toUpperCase().padStart(8, '0')}</p>
              </div>

              {/* Exacto */}
              <div className="bg-gray-800 border border-gray-700 rounded-lg p-4">
                <h3 className="text-yellow-400 font-semibold mb-2">Exacto</h3>
                <p className="text-white font-mono">0x{togenResult.exacto.toString(16).toUpperCase().padStart(12, '0')}</p>
              </div>
            </div>
          </div>
        )}

        {/* Footer */}
        <div className="mt-12 text-center text-gray-500 text-sm">
          <p>Genesis Togenizer v1.1 | Multimodal Hashing Protocol</p>
          <p className="mt-1">Deterministic 128-bit hashing for Text, Code, Images, Audio, and Actions</p>
        </div>
      </div>
    </div>
  )
}
