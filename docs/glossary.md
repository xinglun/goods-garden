# Glossary

## English

| Term | Definition |
| --- | --- |
| Goods | An individual product represented in a business context. |
| Goods Intelligence | The technical kernel for sensing, understanding, asking, remembering and learning. |
| Goods Garden | The product world experienced by people. |
| Living Entity | A product/domain model with state, needs, care, outcomes and learning; not a consciousness claim. |
| State | A representation of a good's current business condition. |
| Expectation | A profile-derived boundary used to assess the current state. |
| Health Assessment | An explainable healthy/unhealthy result derived from an observation and expectation. |
| Need | An explainable condition that calls for care or investigation. |
| Care | Human or system-supported response to a good's need, subject to authority. |
| Caregiver | A human participant who can understand, decide and provide care. |
| Observation | Sensory input from a source. |
| Evidence | Traceable support for an observation, inference or decision. |
| Memory | A record of relevant past experience and relationships. |
| Learning | A reviewable change derived from verified outcomes. |
| Habitat | The store or operating context in which a good exists. |
| Species | A reusable class of goods profiles or capabilities. |
| Individual | One good instance with its own identity and memory. |

## 日本語

| Term | Definition |
| --- | --- |
| Goods | business context で表現される個別の商品。 |
| Goods Intelligence | sensing、understanding、asking、remembering、learning のための technical kernel。 |
| Goods Garden | 人が体験する product world。 |
| Living Entity | state、need、care、outcome、learning を持つ product/domain model。consciousness の主張ではない。 |
| State | 商品の現在の business condition の表現。 |
| Expectation | current state を assess するための profile 由来の boundary。 |
| Health Assessment | observation と expectation から導く explainable な healthy/unhealthy result。 |
| Need | care または investigation を必要とする explainable な condition。 |
| Deviation | ある次元での observation と expectation の間の定量化された gap。 |
| Urgency | Need がどれだけ早く注意を必要とするかを示す explainable な段階的 level。 |
| Need Conflict | 相反する方向を示唆する2つの同時 Need の間の explainable な矛盾。解決も行動の推奨も行わず提示するだけ。 |
| Care | authority に従って need に応答する human または system-supported response。 |
| Care Request | good 単独では need を解決できない時に発生する explainable な request。決定も行動も行わない。 |
| Human Feedback | Care Request に応じて得られる human input。domain はこれを発明・推論・合成しない。 |
| Care Action | Care Request とそれを解決した Human Feedback を結びつける traceable な記録。 |
| Caregiver | 理解し、判断し、care を提供できる human participant。 |
| Observation | source からの sensory input。 |
| Evidence | Information State（`Known`/`Inferred`/`Unknown`/`Unavailable`/`Conflicting`）付きの traceable な statement。observation、inference、decision を支える。 |
| Information State | Evidence が持つ信頼度区分：`Known`/`Inferred`/`Unknown`/`Unavailable`/`Conflicting`。 |
| Memory | 過去の experience と relationship の relevant record。 |
| Memory Record | 1つの記憶された Care episode。Need を促した State と、それに応答した Care Action。効果があったかは判断しない。 |
| Outcome | Care Action が対象とした Need が follow-up observation でまだ存在するかを比較する、事実に基づく判定。 |
| Learning | verified Outcome から導く reviewable な statement。閾値や profile field などのルールを自ら調整することはない。 |
| Habitat | 商品が存在する store または operating context。 |
| Species | goods profile または capability の再利用可能な class。 |
| Individual | 固有の identity と memory を持つ一つの商品 instance。 |
| Lifecycle State | Goods individual が Active（監視中）か Retired（監視対象外）かを示す2値。`Goods::retire()` によってのみ遷移する。 |
| Intelligence Loop | State → Need → Care → Action → New State → Memory/Learning の1周分のサイクル。`GoodsRuntime::run_cycle` として実装される。 |
| Scheduler | Intelligence Loop を人間の介在なしに自動で進める、型レベルで synthetic のみに限定・回数上限・ログ・フォアグラウンドのみに制限された仕組み。`goods_runtime::scheduler::run_scheduled` として実装される。 |

## 中文

| 术语 | 定义 |
| --- | --- |
| Goods | 在经营语境中表示的单个商品。 |
| Goods Intelligence | 用于感知、理解、求助、记忆和学习的技术内核。 |
| Goods Garden | 人所体验的产品世界。 |
| Living Entity | 具有状态、需求、照料、结果和学习的产品/领域模型；不代表意识主张。 |
| State | 商品当前经营状态的表示。 |
| Expectation | 从 profile 得到、用于评估当前状态的边界。 |
| Health Assessment | 根据 observation 和 expectation 得出的可解释 healthy/unhealthy 结果。 |
| Need | 需要照料或调查的可解释条件。 |
| Care | 受权限约束、对商品需求作出的人工或系统支持响应。 |
| Caregiver | 能够理解、决定并提供照料的人类参与者。 |
| Observation | 来自某个来源的感官输入。 |
| Evidence | 支持观察、推断或决定的可追溯依据。 |
| Memory | 相关过去经验和关系的记录。 |
| Learning | 从已验证结果中得到的、可 review 的变化。 |
| Habitat | 商品存在的门店或运营环境。 |
| Species | 可复用的商品 profile 或能力类别。 |
| Individual | 拥有自身身份和记忆的一个商品实例。 |
