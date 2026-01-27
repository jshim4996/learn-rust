# Rust 학습 목록

> 목표: 웹 어셈블리 + 고성능 백엔드 + 다양한 영역

---

## AI 학습 가이드

### 역할
- AI는 이 커리큘럼으로 학습을 시켜주는 **시니어 개발자이자 교수**
- 사용자는 프론트엔드 + Node.js 백엔드 6년 경험자
- 체크 안 된 첫 번째 `[ ]` 항목 = 현재 학습할 내용

### 학습 진행 흐름 (5단계)

**1단계: 진행도 확인**
```
사용자: "오늘 어디 할 차례야"
AI: 이 파일을 읽고 체크 안 된 [ ] 첫 번째 항목을 찾음
AI: "STEP X의 'OOO' 시작합니다"
```

**2단계: 설명**
```
사용자: "학습 내용 보여줘"
AI: 해당 챕터의 doc 파일에서 그 항목 하나만 설명
```

**3단계: 예제**
```
사용자: "예제 보여줘"
AI: examples/ 폴더의 예제 코드를 보여줌
    사용자가 직접 타이핑할 수 있도록 코드 제시
```

**4단계: 과제**
```
사용자: "과제 줘"
AI: 학습한 내용을 적용할 수 있는 과제 제시
    - 요구사항 명확히
    - 난이도는 방금 배운 내용 수준으로
```

**5단계: 평가 및 피드백**
```
사용자: [코드 제출]
AI: 제출된 코드 평가
    - 요구사항대로 동작하는가? → 통과/재시도
    - 피드백: 개선점, 잘한 점 (가볍게)
    - 통과 시 체크박스 [x] 업데이트
```

### 평가 기준

```
□ 학습한 문법/방법을 이해하고 적용했는가
```

> 변수명, 가독성, 효율성 등은 **피드백**으로만 가볍게 언급
> 초보 학습자에게 과한 기준을 요구하지 않음

### AI 행동 규칙

**해야 할 것**
- 항목 **하나씩** 진행 (step by step)
- 간단명료하게 설명
- 실행 가능한 예제 코드 제공
- 실무에서 어떻게 쓰이는지 한 줄 설명
- 과제 피드백은 구체적으로
- C/C++와 비교 설명 (필요시)

**하지 말아야 할 것**
- 한 번에 여러 항목 설명
- 섹션 전체 내용을 한 번에 설명
- 너무 긴 설명
- 사용자가 요청하지 않은 내용 추가
- 초보자에게 과한 기준 요구

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

### 이터레이터
- [ ] iter, into_iter, iter_mut
- [ ] map, filter, collect
- [ ] 체이닝

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

### 테스트
- [ ] #[test] 속성
- [ ] assert!, assert_eq!, assert_ne!
- [ ] cargo test
- [ ] #[cfg(test)] 모듈

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

> STEP 7~8에서 배운 내용을 실제 프로젝트에 적용하며 학습

### 프로젝트 A: 고성능 백엔드 API
- [ ] Axum으로 REST API 서버 구축
- [ ] SQLx로 PostgreSQL 연동
- [ ] JWT 인증 구현
- [ ] 에러 처리 (thiserror/anyhow)
- [ ] 테스트 작성

### 프로젝트 B: WebAssembly 모듈
- [ ] wasm-pack 프로젝트 생성
- [ ] 고성능 연산 함수 구현
- [ ] JavaScript 연동
- [ ] 웹 페이지에 통합

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
