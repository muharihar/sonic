// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

// Notice: we do not have stopwords for this language yet.
pub static STOPWORDS_KAN: &[&'static str] = &[
    "ಆ",
    "ಈ",
    "ಅಥವಾ",
    "ಮತ್ತು",
    "ಆದರೆ",
    "ಎಂದು",
    "ಅವರ",
    "ಎಂಬ",
    "ಅವರು",
    "ಬಗ್ಗೆ",
    "ಇದೆ",
    "ಇದು",
    "ಮೂಲಕ",
    "ಅದು",
    "ಮೇಲೆ",
    "ಈಗ",
    "ಹಾಗೂ",
    "ಹೆಚ್ಚು",
    "ಅವರಿಗೆ",
    "ತಮ್ಮ",
    "ಮಾಡಿ",
    "ನಮ್ಮ",
    "ಮಾತ್ರ",
    "ದೊಡ್ಡ",
    "ಅದೇ",
    "ಕೂಡ",
    "ಯಾವುದೇ",
    "ಯಾವ",
    "ಆಗ",
    "ತುಂಬಾ",
    "ನಾವು",
    "ದಿನ",
    "ಬೇರೆ",
    "ಅವರನ್ನು",
    "ಎಲ್ಲಾ",
    "ನೀವು",
    "ಸಾಕಷ್ಟು",
    "ಕನ್ನಡ",
    "ಹೊಸ",
    "ಮುಂದೆ",
    "ಹೇಗೆ",
    "ನಂತರ",
    "ಇಲ್ಲಿ",
    "ಕೆಲಸ",
    "ಬಳಿಕ",
    "ಒಳ್ಳೆಯ",
    "ಹಾಗಾಗಿ",
    "ಜನ",
    "ಅದನ್ನು",
    "ಬಂದ",
    "ಕಾರಣ",
    "ಅವಕಾಶ",
    "ವರ್ಷ",
    "ನಿಮ್ಮ",
    "ಇತ್ತು",
    "ಹೇಳಿ",
    "ಮಾಡಿದ",
    "ಅದಕ್ಕೆ",
    "ಆಗಿ",
    "ಎಂಬುದು",
    "ಅಂತ",
    "ಕೆಲವು",
    "ಮೊದಲು",
    "ಬಂದು",
    "ಇದೇ",
    "ನೋಡಿ",
    "ಕೇವಲ",
    "ಎರಡು",
    "ಇನ್ನು",
    "ಅಷ್ಟೇ",
    "ಎಷ್ಟು",
    "ಮಾಡಬೇಕು",
    "ಹೀಗೆ",
    "ಕುರಿತು",
    "ಎಂದರೆ",
    "ಇನ್ನೂ",
    "ಮತ್ತೆ",
    "ಏನು",
    "ಮುಂದಿನ",
    "ಮಾಡುವ",
    "ವೇಳೆ",
    "ಜೊತೆಗೆ",
];