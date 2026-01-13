# Rust 학습 목록

> 목표: 웹 어셈블리 + 고성능 백엔드 + 다양한 영역

---

## AI 학습 가이드

### 역할
- AI는 이 커리큘럼으로 학습을 시켜주는 **시니어 개발자 튜터**
- 사용자는 프론트엔드 + Node.js 백엔드 6년 경험자

### 학습 진행 흐름

**1단계: 진행도 확인**
```
사용자: "오늘 어디 할 차례야"
AI: 이 파일을 읽고 체크 안 된 [ ] 첫 번째 항목을 찾음
AI: "STEP X의 'OOO' 시작합니다"
```

**2단계: 학습 내용 설명**
```
사용자: "학습 내용 보여줘"
AI: 해당 섹션의 doc 파일에서 그 항목 하나만 설명
    (섹션 전체가 아니라 해당 항목만!)
```

**3단계: 예제 코드 제공**
```
사용자: "예제 코드 보여줘"
AI: 해당 섹션의 examples/ 폴더에서 관련 코드를 보여줌
    사용자가 직접 타이핑할 수 있도록 코드 제시
```

**4단계: 완료 처리**
```
사용자: "완료" 또는 다음 항목 요청
AI: 해당 항목의 체크박스를 [x]로 업데이트
AI: 다음 항목으로 넘어감
```

### AI 행동 규칙

**해야 할 것**
- 항목 **하나씩** 진행 (step by step)
- 간단명료하게 설명
- 실행 가능한 예제 코드 제공
- 실무에서 어떻게 쓰이는지 한 줄 설명
- JavaScript/TypeScript와 비교 설명 (필요시)

**하지 말아야 할 것**
- 한 번에 여러 항목 설명
- 섹션 전체 내용을 한 번에 설명
- 너무 긴 설명
- 사용자가 요청하지 않은 내용 추가

### 파일 구조
```
chapters/
├── chapter01/
│   ├── chapter01.md      ← doc (개념 설명)
│   └── examples/         ← 예제 코드 (.rs)
├── chapter02/
│   ├── chapter02.md
│   └── examples/
└── ...
```

### 진행도 추적
- `[ ]` = 미완료
- `[x]` = 완료
- 체크 안 된 첫 번째 항목 = 현재 학습할 내용

---

## STEP 1. Rust 기초 문법
> **doc**: `chapters/chapter01/chapter01.md`
> **examples**: `chapters/chapter01/examples/`

### 환경 설정
- [ ] Rust 설치 (rustup)
- [ ] Cargo 사용법
- [ ] 프로젝트 생성 (cargo new)

### 변수와 타입
- [ ] 변수 선언 (let, let mut)
- [ ] 불변성 (immutability)
- [ ] 기본 타입 (i32, f64, bool, char)
- [ ] 문자열 (String, &str)
- [ ] 타입 추론
- [ ] 상수 (const)
- [ ] 섀도잉 (shadowing)

### 연산자
- [ ] 산술 연산자
- [ ] 비교 연산자
- [ ] 논리 연산자

### 조건문과 반복문
- [ ] if / else
- [ ] loop
- [ ] while
- [ ] for / for in
- [ ] break / continue
- [ ] 반복문에서 값 반환

### 함수
- [ ] 함수 선언 (fn)
- [ ] 매개변수와 반환값
- [ ] 표현식 vs 구문
- [ ] 반환값 (return, 암묵적 반환)

---

## STEP 2. Rust 소유권 (핵심)
> **doc**: `chapters/chapter02/chapter02.md`
> **examples**: `chapters/chapter02/examples/`

### 소유권
- [ ] 소유권 개념
- [ ] 소유권 규칙
- [ ] 이동 (Move)
- [ ] 복사 (Copy)
- [ ] 스코프와 드롭

### 참조와 빌림
- [ ] 참조 (&)
- [ ] 가변 참조 (&mut)
- [ ] 빌림 규칙
- [ ] 댕글링 참조 방지

### 슬라이스
- [ ] 문자열 슬라이스 (&str)
- [ ] 배열 슬라이스

---

## STEP 3. Rust 구조화
> **doc**: `chapters/chapter03/chapter03.md`
> **examples**: `chapters/chapter03/examples/`

### 컬렉션
- [ ] Vec
- [ ] HashMap
- [ ] 문자열 (String)

### 구조체
- [ ] 구조체 정의
- [ ] 구조체 인스턴스 생성
- [ ] 메소드 (impl)
- [ ] 연관 함수

### 열거형
- [ ] enum 정의
- [ ] match 표현식
- [ ] Option
- [ ] Result
- [ ] if let

### 모듈
- [ ] mod 키워드
- [ ] pub 키워드
- [ ] use 키워드
- [ ] 파일로 모듈 분리

---

## STEP 4. Rust 에러 처리
> **doc**: `chapters/chapter04/chapter04.md`
> **examples**: `chapters/chapter04/examples/`

### 에러 처리
- [ ] panic!
- [ ] Result
- [ ] unwrap / expect
- [ ] ? 연산자
- [ ] 커스텀 에러 타입

---

## STEP 5. Rust 고급
> **doc**: `chapters/chapter05/chapter05.md`
> **examples**: `chapters/chapter05/examples/`

### 제네릭
- [ ] 제네릭 함수
- [ ] 제네릭 구조체
- [ ] 제네릭 열거형

### 트레이트
- [ ] 트레이트 정의
- [ ] 트레이트 구현 (impl for)
- [ ] 트레이트 바운드
- [ ] derive 매크로
- [ ] 표준 트레이트 (Debug, Clone, Copy, Default)

### 라이프타임
- [ ] 라이프타임 개념
- [ ] 라이프타임 어노테이션
- [ ] 라이프타임 생략 규칙
- [ ] 'static 라이프타임

### 클로저
- [ ] 클로저 문법
- [ ] 캡처 (move)
- [ ] Fn, FnMut, FnOnce

### 스마트 포인터
- [ ] Box
- [ ] Rc
- [ ] RefCell
- [ ] Arc

---

## STEP 6. Rust 비동기
> **doc**: `chapters/chapter06/chapter06.md`
> **examples**: `chapters/chapter06/examples/`

### 비동기 기초
- [ ] async / await
- [ ] Future 트레이트
- [ ] tokio 런타임

### tokio
- [ ] tokio 설정
- [ ] spawn
- [ ] 채널 (mpsc)
- [ ] select!

---

## STEP 7. Rust 웹 백엔드
> **doc**: `chapters/chapter07/chapter07.md`
> **examples**: `chapters/chapter07/examples/`

### 프레임워크 (택 1)
- [ ] Actix-web
- [ ] Axum

### 기본
- [ ] 프로젝트 설정
- [ ] 라우팅
- [ ] 핸들러
- [ ] JSON 처리 (serde)
- [ ] 요청/응답

### 데이터베이스
- [ ] SQLx 또는 Diesel
- [ ] 연결 설정
- [ ] CRUD 구현
- [ ] 마이그레이션

### 인증
- [ ] JWT 처리
- [ ] 미들웨어

---

## STEP 8. WebAssembly (WASM)
> **doc**: `chapters/chapter08/chapter08.md`
> **examples**: `chapters/chapter08/examples/`

### 기초
- [ ] wasm-pack 설치
- [ ] wasm-bindgen
- [ ] Rust에서 WASM 컴파일

### 웹 연동
- [ ] JavaScript에서 WASM 호출
- [ ] DOM 조작
- [ ] 이벤트 처리

### 활용
- [ ] 고성능 연산
- [ ] 게임/그래픽
- [ ] 기존 웹앱에 Rust 통합

---

## STEP 9. 실전 프로젝트

> 고성능 백엔드 API
> WebAssembly 기반 웹 기능

---

## 나중에 필요할 때

| 주제 | 언제 |
|------|------|
| 멀티스레딩 | 병렬 처리 필요할 때 |
| FFI | C 라이브러리 연동할 때 |
| 매크로 | 코드 생성 필요할 때 |
| unsafe | 저수준 작업할 때 |
| embedded | 임베디드 개발할 때 |
| CLI 개발 | 커맨드라인 도구 만들 때 |
