// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod memory_cache;
mod metrics;

pub use memory_cache::new_memory_cache;
pub use memory_cache::MemoryCache;
pub use memory_cache::SegmentInfoCache;
pub use memory_cache::TableSnapshotCache;

pub use self::metrics::CacheDeferMetrics;
pub use self::metrics::TenantLabel;
