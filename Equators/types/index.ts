export interface FileInfo {
  name: string
  type: string
  size: number
}

export interface Message {
  id: string
  role: "user" | "assistant" | "system"
  content: string
  timestamp: string
  files?: FileInfo[]
  isCommand?: boolean
  isError?: boolean
}

export interface Conversation {
  id: string
  title: string
  messages: Message[]
}

