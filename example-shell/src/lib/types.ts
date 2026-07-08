export interface MprisPlayer {
	serviceName: string
	identity: string
	playbackStatus: string
	title: string
	artist: string
	album: string
	artUrl: string
	position: number
	length: number
	canPlay: boolean
	canPause: boolean
	canGoNext: boolean
	canGoPrevious: boolean
}

export interface NiriWorkspace {
	id: number
	idx: number
	name: string | null
	output: string | null
	is_urgent: boolean
	is_active: boolean
	is_focused: boolean
	active_window_id: number | null
}

export interface SystemStatus {
	battery: {
		capacity: number | null
		charging: boolean
	}
	online: boolean
}
