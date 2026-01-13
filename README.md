# learn-rust

> AI가 가르치고, 사람이 배운다 - Rust 시스템 프로그래밍 학습 

---

## 프로젝트 목표

**최종 목표**: WebAssembly + 고성능 백엔드 + 시스템 프로그래밍

### 학습 범위
1. **Rust 기초** - 변수, 타입, 함수, 조건문, 반복문
2. **소유권** - 소유권, 참조, 빌림, 슬라이스 (Rust 핵심)
3. **구조화** - 컬렉션, 구조체, 열거형, 모듈
4. **에러 처리** - Result, Option, ? 연산자
5. **고급** - 제네릭, 트레이트, 라이프타임, 클로저
6. **비동기** - async/await, tokio
7. **웹 백엔드** - Actix-web/Axum, SQLx, JWT
8. **WebAssembly** - wasm-pack, wasm-bindgen

---

## 학습 방식

이 프로젝트는 **AI 튜터 기반 학습 실험**입니다.

- AI가 커리큘럼에 따라 단계별로 설명
- 한 번에 하나의 항목만 학습
- 체크리스트로 진행 상황 추적
- 실행 가능한 예제 코드 제공
- JavaScript/TypeScript와 비교 설명

### 사용법
```
사용자: "오늘 어디 할 차례야"
AI: PROGRESS.md를 읽고 다음 학습 항목 안내
```

---

## 폴더 구조

```
learn-rust/
├── README.md           ← 프로젝트 소개
├── PROGRESS.md         ← 학습 체크리스트 + AI 가이드
└── chapters/
    ├── chapter01/
    │   ├── chapter01.md    ← 개념 설명
    │   └── examples/       ← 예제 코드 (.rs)
    ├── chapter02/
    └── ...
```

---

## 진행 상황

`PROGRESS.md` 파일에서 확인

- `[ ]` = 미완료
- `[x]` = 완료

---

## 환경

- **언어**: Rust (stable)
- **패키지 매니저**: Cargo
- **런타임**: tokio (비동기)
- **AI**: Claude (Anthropic)
