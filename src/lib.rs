pub mod tts {
    use windows::{
        core::HSTRING,
        Win32::Media::Speech::{ISpVoice, SpVoice},
        Win32::System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL},
    };
    
    pub fn tts(text : &str) {
        unsafe {
            let _ = CoInitialize(None); // COM 라이브러리 초기화
            let voice: ISpVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL).unwrap();
            voice.SetRate(-3).unwrap();
    
            // 텍스트 음성 변환
            let text = HSTRING::from(text);
            voice.Speak(&text, 0, None).unwrap();
    
            CoUninitialize();   // Com 라이브러리 해제 
        }
    }

    #[cfg(test)]
    pub mod test {
        use super::tts;

        #[test]
        fn test_tts() {
            // let text_list : Vec<&str> = vec!["사과", "포도", "파인애플", "바나나", "딸기"];
            // for txt in text_list {
            //     tts(txt);
            // }

            tts("김우용");   
        }
    }
}

