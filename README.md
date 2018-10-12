Usage:

```
cargo run ./sample.json ./student.yaml
```

---

At a high level, `@gob/hanson-format` will take this:

```yaml
name: Asian Studies
type: major
revision: 2012-13

result: Requirement & CSCI 121

Requirement:
  Asian Studies:
    result: ASIAN 130

  Dance:
    result: DANCE 101

   result: all of (Asian Studies, Dance)
```

and first output this:

```yaml
name: Asian Studies
type: major
revision: 2012-13

result: Requirement & CSCI 121

requirements:
  - name: Requirement
    result: all of (Asian Studies, Dance)
    requirements:
      - name: Asian Studies
        result: ASIAN 130
      - name: Dance
        result: DANCE 101
```

and then expand each `result` and `filter` into the expanded data structures, resulting in:

```yaml
name: Asian Studies
type: major
revision: 2012-13

result:
  type: boolean-and
  values:
    - type: reference
      requirement: Requirement
    - type: course
      department: CSCI
      number: 121

requirements:
  - name: Requirement
    result:
      type: of
      counter: {num: null, was: 'all', operator: Eq}
      values:
        - type: reference
          requirement: Asian Studies
        - type: reference
          requirement: Dance
    requirements:
      - name: Asian Studies
        result:
          type: course
          department: ASIAN
          number: 130
      - name: Dance
        result:
          type: course
          department: DANCE
          number: 101
```

That YAML is then converted into JSON, which is

Then, `@gob/e
